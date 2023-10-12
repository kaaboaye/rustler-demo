use csv::Reader;
use rstar::{Point, RTree};
use rustler::NifMap;
use std::{collections::HashMap, path::PathBuf};

use crate::streets::{read_streets_csv, Street};

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct CsvHome {
    pub point_id: String,
    pub street_id: String,
    pub postal_code: String,
    pub home_number: String,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Clone, PartialEq, NifMap)]
pub struct Home {
    pub point_id: String,
    pub street_id: String,
    pub street_prefix: String,
    pub street_name: String,
    pub postal_code: String,
    pub home_number: String,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HomePoint {
    pub latitude: f32,
    pub longitude: f32,
    pub home: Option<&'static Home>,
}

impl HomePoint {
    pub fn empty(longitude: f32, latitude: f32) -> Self {
        Self {
            longitude,
            latitude,
            home: None,
        }
    }
}

impl Point for HomePoint {
    type Scalar = f32;
    const DIMENSIONS: usize = 2;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        HomePoint::empty(generator(0), generator(1))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.longitude,
            1 => self.latitude,
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.longitude,
            1 => &mut self.latitude,
            _ => unreachable!(),
        }
    }
}

static mut HOMES_VEC: Option<&'static [Home]> = None;
static mut HOMES_IDX: Option<RTree<HomePoint>> = None;

pub fn load_homes(path: PathBuf) {
    let csv_streets: HashMap<String, Street> = read_streets_csv(path.clone())
        .into_iter()
        .map(|street| (street.street_id.clone(), street))
        .collect();

    let csv_homes = read_csv(path);

    let homes: Vec<Home> = csv_homes
        .into_iter()
        .map(move |csv_home| {
            let street = csv_streets.get(&csv_home.street_id).unwrap();

            Home {
                point_id: csv_home.point_id,
                street_id: csv_home.street_id,
                street_prefix: street.street_prefix.clone(),
                street_name: street.street_name.clone(),
                postal_code: csv_home.postal_code,
                home_number: csv_home.home_number,
                latitude: csv_home.latitude,
                longitude: csv_home.longitude,
            }
        })
        .collect();

    let homes: &'static [Home] = Box::leak(homes.into_boxed_slice());

    let idx: RTree<HomePoint> = RTree::bulk_load(
        homes
            .iter()
            .map(|home| {
                let point = HomePoint {
                    latitude: home.latitude,
                    longitude: home.longitude,
                    home: Some(home),
                };
                point
            })
            .collect(),
    );

    unsafe { HOMES_VEC = Some(homes) };
    unsafe { HOMES_IDX = Some(idx) };
}

fn read_csv(mut path: PathBuf) -> Vec<CsvHome> {
    path.push("mieszkania_wroclaw.csv");
    eprintln!("Reading homes from: {:?}", &path);

    let reader = Reader::from_path(path).unwrap();
    let data = reader
        .into_deserialize::<CsvHome>()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for record in data.iter().take(3) {
        eprintln!("{:?}", record);
    }

    eprintln!("Homes loaded");

    return data;
}

fn get_idx() -> &'static RTree<HomePoint> {
    unsafe { HOMES_IDX.as_ref() }.expect("Did not load homes during startup")
}

pub fn nearest_home(longitude: f32, latitude: f32) -> Option<&'static Home> {
    get_idx()
        .nearest_neighbor(&HomePoint::empty(longitude, latitude))?
        .home
}
