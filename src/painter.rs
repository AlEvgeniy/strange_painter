use std::sync::atomic::{Ordering, AtomicUsize};

use rayon::prelude::*;

use super::{Pal, Input};

fn do_cut(left: f32, top: f32, right: f32, bottom: f32, z: num_complex::Complex32, cut: bool) -> num_complex::Complex32
{
    if !cut
    {
        z
    }
    else
    {
        let res_x: f32 = if z.re > right
        {
            left + z.re - ((z.re - left)/(right - left)).trunc()
        }
        else if z.re < left
        {
            let xx: f32 = left + left - z.re;
            left + xx - ((xx - left)/(right - left)).trunc()
        }
        else
        {
            z.re
        };

        let res_y: f32 =  if z.im > top
        {
            bottom + z.im - ((z.im - bottom)/(top - bottom)).trunc()
        }
        else if z.im < bottom
        {
            let yy: f32 = bottom + bottom - z.im;
            bottom + yy - ((yy - bottom)/(top - bottom)).trunc()
        }
        else
        {
            z.im
        };

        num_complex::Complex32::new(res_x, res_y)
    }
}

pub fn paint(inp: &Input, width: usize, height: usize, palet: &Pal, depth: usize) -> Vec<u8>
{
    println!("Painting started...");
    let start = std::time::Instant::now();
    //let mut field: Vec<usize> = vec![1 ; width*height];
    let field: Vec<AtomicUsize> = (0..width*height).map(|_| AtomicUsize::new(1)).collect();
    let mut nxt_field: Vec<(f32, f32)> = (0..field.len()).map(
        |i|
        {
            let int_x: usize = i % width;
            let int_y: usize = i / width;

            let f_x: f32 = inp.left + (inp.right - inp.left)*((width - 1) as f32)/int_x as f32;
            let f_y: f32 = inp.bottom + (inp.top - inp.bottom)*((height - 1) as f32)/(height - int_y - 1) as f32;

            (f_x, f_y)
        }
    ).collect();

    let pb = indicatif::ProgressBar::new(depth as u64);

    for _ in 0..depth
    {
        nxt_field = nxt_field.par_iter().map(
            |&(f_x, f_y)|
            {
                let res: num_complex::Complex32 = inp.transforms.iter().fold(num_complex::Complex32::new(f_x, f_y),
                    |z, trans|
                    {
                        let num: num_complex::Complex32 = trans.numerator_coefs.iter().rev().fold(num_complex::Complex32::new(0.0, 0.0),
                            |acc, (px, py)|
                            {
                                let c: num_complex::Complex32 = num_complex::Complex32::new(*px, *py);
                                do_cut(inp.left, inp.top, inp.right, inp.bottom, c + do_cut(inp.left, inp.top, inp.right, inp.bottom, acc*z, trans.cut), trans.cut)
                            }
                        );

                        let den: num_complex::Complex32 = if trans.denominator_coefs.len() == 0
                        {
                            num_complex::Complex32::new(1.0, 0.0)
                        }
                        else
                        {
                            trans.denominator_coefs.iter().rev().fold(num_complex::Complex32::new(0.0, 0.0),
                                |acc, (px, py)|
                                {
                                    let c: num_complex::Complex32 = num_complex::Complex32::new(*px, *py);
                                    do_cut(inp.left, inp.top, inp.right, inp.bottom, c + do_cut(inp.left, inp.top, inp.right, inp.bottom, acc*z, trans.cut), trans.cut)
                                }
                            )
                        };

                        do_cut(inp.left, inp.top, inp.right, inp.bottom, num/den, trans.cut)
                    }
                );

                if (res.re >= inp.left) && (res.re <= inp.right) && (res.im >= inp.bottom) && (res.im <= inp.top)
                {
                    let res_int_x: usize = ((res.re - inp.left)*(width - 1) as f32/(inp.right - inp.left)) as usize;
                    let res_int_y: usize = height - ((res.im - inp.bottom)*(height - 1) as f32/(inp.top - inp.bottom)) as usize - 1;
                    field[res_int_x + res_int_y*width].fetch_add(1, Ordering::Relaxed);
                }

                (res.re, res.im)
            }
        ).collect();

        pb.inc(1);
    }

    pb.finish();

    println!("Log...");

    let log_field: Vec<f32> = field.iter().map(
        |f|
        {
            (f.load(Ordering::Relaxed) as f32).ln()
        }
    ).collect();

    println!("Getting max...");

    let the_log_max: f32 = log_field.iter().fold(0.0, |acc, &f| if f > acc {f} else {acc});

    println!("Ending...");

    let res: Vec<(u8, u8, u8)> = log_field.into_iter().map(|f| palet.get_col(f, the_log_max)).collect();

    let e = (0..3*res.len()).map(
        |i|
        {
            match i % 3
            {
                0 => res[i/3].0,
                1 => res[i/3].1,
                _ => res[i/3].2,
            }
        }
    ).collect();

    println!("Painted in {:?}", start.elapsed());

    e
}