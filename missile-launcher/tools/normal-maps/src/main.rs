use image::{Rgb, RgbImage};
use std::{error::Error, fs, path::Path};

// Numeric tangent-space data, generated independently of painted colour/lettering.
// Positive Y follows increasing atlas V, including the project's top-left UV origin.
fn noise(x: f64, y: f64, seed: u32) -> f64 {
    fn hash(x: i32, y: i32, seed: u32) -> f64 {
        let mut n = (x as u32).wrapping_mul(374761393)
            ^ (y as u32).wrapping_mul(668265263) ^ seed;
        n = (n ^ (n >> 13)).wrapping_mul(1274126177);
        ((n ^ (n >> 16)) as f64 / u32::MAX as f64) * 2.0 - 1.0
    }
    let (ix, iy) = (x.floor() as i32, y.floor() as i32);
    let (u, v) = (x - x.floor(), y - y.floor());
    let (u, v) = (u*u*(3.0-2.0*u), v*v*(3.0-2.0*v));
    let a = hash(ix, iy, seed)*(1.0-u) + hash(ix+1, iy, seed)*u;
    let b = hash(ix, iy+1, seed)*(1.0-u) + hash(ix+1, iy+1, seed)*u;
    a*(1.0-v)+b*v
}

fn height(x: f64, y: f64, kind: &str, seed: u32) -> f64 {
    let grain = noise(x/2.1, y/2.1, seed);
    match kind {
        "metal" => 0.075*(y*2.1 + noise(x/50.0,y/8.0,seed)).sin()
            + 0.022*grain + 0.018*noise(x/14.0,y/14.0,seed+1),
        "composite" => 0.075*(x*1.4).sin()*(y*1.4).sin() + 0.025*grain,
        "rubber" => 0.13*grain + 0.045*noise(x/7.0,y/7.0,seed+1),
        "glass" => 0.003*grain,
        _ => 0.12*grain + 0.065*noise(x/5.0,y/5.0,seed+1)
            - 0.2*(noise(x/9.0,y/9.0,seed+2)-0.45).max(0.0).powi(2),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("compare") {
        let a = image::open(&args[2])?.to_rgb8();
        let b = image::open(&args[3])?.to_rgb8();
        assert_eq!(a.dimensions(), b.dimensions());
        let changed = a.pixels().zip(b.pixels()).filter(|(a,b)| a != b).count();
        let error: u64 = a.as_raw().iter().zip(b.as_raw()).map(|(a,b)| a.abs_diff(*b) as u64).sum();
        println!("Changed pixels: {changed}; mean channel difference: {:.4}", error as f64/a.as_raw().len() as f64);
        assert!(changed > 100, "Normal maps must change rendered shading");
        return Ok(());
    }
    let root = Path::new(args.get(1).ok_or("supply asset project directory")?);
    let asset = fs::read_to_string(root.join("platform.gasset"))?;
    let mut bindings = String::new();
    let mut report = String::from("source,finish,resolution,normal_convention\n");
    let mut count = 0;
    for line in asset.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len()<4 || fields[0]!="map" || fields[2]!="base_colour" {continue;}
        let source = fields[1];
        let name = source.trim_start_matches("paint_").trim_start_matches("finish_");
        let kind = match name {
            "jacks"|"pivots"|"collars"|"hubs"|"rearplates"|"fasteners"|"latch" => "metal",
            "payload-main"|"payload-nose" => "composite",
            "screenframe" => "rubber",
            "display"|"lenses" => "glass",
            _ => "paint",
        };
        let seed = source.bytes().fold(2166136261u32, |s,b| (s^b as u32).wrapping_mul(16777619)) & 0x7fffffff;
        let mut map = RgbImage::new(512,512);
        for (x,y,pixel) in map.enumerate_pixels_mut() {
            let (u,v)=(x as f64,y as f64);
            let nx=-(height(u+0.5,v,kind,seed)-height(u-0.5,v,kind,seed));
            let ny=-(height(u,v+0.5,kind,seed)-height(u,v-0.5,kind,seed));
            let length=(nx*nx+ny*ny+1.0).sqrt();
            *pixel=Rgb([nx,ny,1.0].map(|c| ((c/length*0.5+0.5)*255.0).round() as u8));
        }
        let filename=format!("textures/{name}-normal.png");
        map.save(root.join(&filename))?;
        bindings.push_str(&format!("map {source} normal {filename} license=LicenseRef-Project-Generated origin=procedural-surface-detail-v1\n"));
        report.push_str(&format!("{source},{kind},512,positive-y\n"));
        count+=1;
    }
    assert_eq!(count,28,"Review material additions before regenerating");
    fs::write(root.join("normal-bindings.gasset.txt"), bindings)?;
    fs::write(root.join("normal-map-manifest.csv"), report)?;
    println!("Generated {count} numeric normal maps and bindings.");
    Ok(())
}
