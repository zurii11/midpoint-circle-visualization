use std::fs::File;
use std::io;
use std::io::Write;
use std::env;


// Explanation
// in PPM format we need to define each pixel with 3 values(RGB),
// each takes 1 byte to store. We do that using hexadecimal values.
// With hex numbers we can use only 1 slot in an array instead of 3 to represent a single pixel.
// Then we iterate over the array, extract RGB values for each pixel and write them to ppm file.
fn write_as_ppm(path: &str, pixels: & [u32], width: usize, height: usize) -> io::Result<()>
{
    let mut file = File::create(path)?;
    write!(file, "P6\n{} {} 255\n", width, height)?;

    for y in 0..height
    {
        for x in 0..width
        {
            let pixel = pixels[y*width+x];                       // [y*width+x] the multiplication and addition are needed because the array
                                                                 // is one dimensional, but we are
                                                                 // operating in 2 dimensions

            let color_buffer = [((pixel >> 8*2) & 0xFF) as u8,   // I'm taking hexadecimal value here, shifting it times to the right and
                                ((pixel >> 8*1) & 0xFF) as u8,   // extracting those values by aplying a mask 0xFF. Remember, each 2 symbol
                                ((pixel >> 8*0) & 0xFF) as u8,]; // represents Red, Green or Blue, so after the first shift we are left with
                                                                 // first 2 symbols(Red), then
                                                                 // second 2 symbols(Green) and the
                                                                 // last 2 symbols(Blue).

            file.write(&color_buffer)?;                          // Here we write RGB values we extracted above
        }
    }
    Ok(())
}

fn stripes_pattern(pixels: &mut [u32], width: usize, height: usize, tile_size: usize, foreground: u32, background: u32)
{
    for y in 0..height
    {
        for x in 0..width
        {
            pixels[y*width+x] = if ((x+y) / tile_size) % 2 == 0
            {
                background
            }
            else
            {
                foreground
            };
        }
    }
}

fn checker_pattern(pixels: &mut [u32], width: usize, height: usize, tile_size: usize, foreground: u32, background: u32)
{
    for y in 0..height
    {
        for x in 0..width
        {
            pixels[y*width+x] = if (x / tile_size + y / tile_size) % 2 == 0
            {
                background
            }
            else
            {
                foreground
            };
        }
    }
}

fn solid_circle(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: u32, background: u32)
{
    let cx = width as i32;
    let cy = height as i32;
    let r = radius as i32 * 2;

    for y in 0..height
    {
        for x in 0..width
        {
            let dx = cx - x as i32 * 2 - 1;
            let dy = cy - y as i32 * 2 - 1;

            pixels[y*width+x] = if dx*dx + dy*dy <= r*r
            {
                background
            }
            else
            {
                foreground
            };
        }
    }
}

fn hollow_circle(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: u32, background: u32)
{
    pixels.fill(background);

    let w = width as f32;
    let h = height as f32;
    let r = radius as f32;
    let cx = w/2.0;
    let cy = h/2.0;
    let mut x = 0.0;
    let mut y = r - 0.5;

    while x <= y
    {
        let px = cx + x;
        let py = cy + y;

        if (0.0..w).contains(&px) && (0.0..h).contains(&py)
        {
            assert!(width == height);

            let dx = px as usize;
            let dy = py as usize;

            pixels[dy * width + dx] = foreground;
            pixels[dx * width + dy] = foreground;

            pixels[(height - dy) * width + dx] = foreground;
            pixels[dy * width + (width - dx)] = foreground;
            
            pixels[dx * width + (height - dy)] = foreground;
            pixels[(width - dx) * width + dy] = foreground;

            pixels[(width - dx) * width + (height - dy)] = foreground;
            pixels[(height - dy) * width + (width - dx)] = foreground;
        }

        x += 1.0;
        if x*x + y*y > r*r
        {
            y -= 1.0;
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;
    const RADIUS: usize = WIDTH/3;
    const TILE_SIZE: usize = 32;
    const FOREGROUND: u32 = 0x00FF00;
    const BACKGROUND: u32 = 0x0000FF;
    const OUTPUT_PATH: &str = "img.ppm";

    let mut pixels = [0u32; WIDTH*HEIGHT];
    pixels.fill(0xFF0000);

    if args.len() > 1
    {
        match &args[1][..]
        {
                "-c" | "--checker" => 
                {
                    checker_pattern(&mut pixels, WIDTH, HEIGHT, TILE_SIZE, FOREGROUND, BACKGROUND);
                    write_as_ppm("checker_pattern.ppm", &pixels, WIDTH, HEIGHT);
                    println!("Writing checker pattern to ppm");
                }, 
                "-s" | "--stripes" =>
                {
                    stripes_pattern(&mut pixels, WIDTH, HEIGHT, TILE_SIZE, FOREGROUND, BACKGROUND);
                    write_as_ppm("stripes_pattern.ppm", &pixels, WIDTH, HEIGHT);
                    println!("Writing stripes pattern to ppm");
                },
                "-sc" | "--solid" =>
                {
                    solid_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND);
                    write_as_ppm("solid_circle.ppm", &pixels, WIDTH, HEIGHT);
                    println!("Writing solid circle to ppm");
                },
                "-hc" | "--hollow" =>
                {
                    hollow_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND);
                    write_as_ppm("hollow_circle.ppm", &pixels, WIDTH, HEIGHT);
                    println!("Writing hollow circle to ppm");
                },
                _ => println!("Invalid argument!"),
        }
    }
    else
    {
        println!("nothing");
        checker_pattern(&mut pixels, WIDTH, HEIGHT, TILE_SIZE, FOREGROUND, BACKGROUND);
        write_as_ppm("checker_pattern.ppm", &pixels, WIDTH, HEIGHT);
        println!("Writing checker pattern to ppm");
        
        stripes_pattern(&mut pixels, WIDTH, HEIGHT, TILE_SIZE, FOREGROUND, BACKGROUND);
        write_as_ppm("stripes_pattern.ppm", &pixels, WIDTH, HEIGHT);
        println!("Writing stripes pattern to ppm");

        solid_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND);
        write_as_ppm("solid_circle.ppm", &pixels, WIDTH, HEIGHT);
        println!("Writing solid circle to ppm");

        hollow_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND);
        write_as_ppm("hollow_circle.ppm", &pixels, WIDTH, HEIGHT);
        println!("Writing hollow circle to ppm");
    }
}
