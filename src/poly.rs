use geo::{LineString, Polygon};
use std::fs;

pub fn load(path: &str) -> Result<Vec<Polygon<f64>>, std::io::Error> {
    debug!("loading poly from path: {}", path);
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    _load(&contents)
}

fn _load(contents: &str) -> Result<Vec<Polygon<f64>>, std::io::Error> {
    let lines = contents.lines();
    let mut mode = 0;
    let mut coords: Vec<(f64, f64)> = Vec::new();
    let mut polygons: Vec<Polygon<f64>> = Vec::new();
    for line in lines {
        let trimed = line.trim_end();
        let replaced = trimed.replace("\t", " ");
        let swt = replaced.starts_with(" ");
        match mode {
            0 => {
                if swt {
                    //begin area
                    mode = 1;
                }
            }
            1 => {
                if swt {
                    let items: Vec<&str> = replaced.trim().split_whitespace().collect();
                    if items.len() == 2 {
                        let coord = (
                            items[0].parse::<f64>().unwrap(),
                            items[1].parse::<f64>().unwrap(),
                        );
                        coords.push(coord);
                    }
                }
                if trimed == "END" {
                    mode = 0;
                    polygons.push(Polygon::<f64>::new(LineString::from(coords), vec![]));
                    coords = vec![];
                }
            }
            _ => {}
        }
    }
    Ok(polygons)
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::algorithm::contains::Contains;
    use geo::Point;

    #[test]
    fn test_load() {
        let content = "freight_tiger_boundary \n\
area1\n\
\t75.81\t38.86\n\
\t68.86\t35.65\n\
\t67.04\t22.21\n\
\t72.38\t6.13\n\
\t77.95\t4.26\n\
\t83.6\t10.36\n\
\t90.68\t18.78\n\
\t99.63\t28.89\n\
\t94.02\t30.58\n\
\t86.49\t28.04\n\
\t81.37\t31.07\n\
\t81.46\t35.96\n\
\t75.81\t38.86\n\
END\n\
END";
        let polygons = _load(content).unwrap();
        println!("{}", content);
        println!("number of polygons parsed: {}", polygons.len());
        let point = Point::<f64>::new(77.1064213, 24.2050449);
        let mut ok = false;
        for polygon in polygons {
            if polygon.contains(&point) {
                ok = true;
                break;
            }
        }
        assert!(ok)
    }
}
