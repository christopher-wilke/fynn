use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]
pub struct Importer {
    v: Vec<Point>,
    pub y_true: Vec<i32>
}

impl Importer {
    pub fn from(f: &str) -> Self {
        log::trace!("Importing '{f}'");
        let file = File::open(f).expect("could not open input file");
        let reader = io::BufReader::new(file).lines();
        let mut resp = vec![];

        for line in reader.flatten() {
            for v in line.split_whitespace() {
                let val: f64 = v.parse().expect("could not parse value");
                resp.push(val);
            }
        }

        let converter = Self::converter(resp);

        Self {
            v: converter.0,
            y_true: converter.1
        }
    }


    fn converter(values: Vec<f64>) -> (Vec<Point>, Vec<i32>) {
        let mut v: Vec<Point> = vec![];
        let mut y_true = vec![];
        let mut current_y_true = 0;
        
        for chunk in values.chunks(2) {
            if let [x, y] = chunk {
                if *x == 0. && *y == 0. {
                    current_y_true += 1;
                }
                v.push(Point { x: *x, y: *y });
                log::error!("{current_y_true:?}");
                y_true.push(current_y_true);
            }
        }

        (v, y_true)
    }

    pub fn get_v(&self) -> &Vec<Point> {
        &self.v
    }

    pub fn get_y_true(&self) -> &Vec<i32> {
        &self.y_true
    }

    // pub fn convert(&mut self) -> Vec<Vec<f64>> {
    //     let mut v = vec![];
    //     let mut cat = 0;
    //     for p in self.v.iter() {
    //         if p.x == 0. && p.y == 0. {
    //             cat += 1;
    //         } 
    //         v.push(vec![p.x, p.y]);
    //         self.y_true.push(cat);
    //     }
    //     v
    // }
}
