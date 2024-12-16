use plotters::prelude::*;
use geo_types::Geometry;
use geojson::GeoJson;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geojson_str = fs::read_to_string("japan.geojson").inspect(|_| println!("ファイルを読み込みました")).expect("ファイルの読み込みに失敗しました");
    let geojson = geojson_str.parse::<GeoJson>().inspect(|_| println!("パースに成功しました")).expect("Geojsonのパースに失敗しました");

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;


    let mut polygons = Vec::new();

    match geojson {
        GeoJson::FeatureCollection(collection) => {
            println!("len: {}", collection.features.len());
            for feature in collection.features {
                if let Some(geometry) = feature.geometry {
                    let geometry: Geometry<f64> = geometry.value.try_into()?;
                    match geometry {
                        Geometry::Polygon(polygon) => {
                            let mut points = Vec::new();
                            for point in polygon.exterior().points() {
                                println!("formatting...");
                                min_x = min_x.min(point.x());
                                max_x = max_x.max(point.x());
                                min_y = min_y.min(point.y());
                                max_y = max_y.max(point.y());
                                points.push((point.x(), point.y()));
                            }
                            polygons.push(points);
                        },
                        Geometry::MultiPolygon(multipolygon) => {
                            for polygon in multipolygon.iter() {
                                let mut points = Vec::new();
                                for point in polygon.exterior().points() {
                                    println!("formatting...");
                                    min_x = min_x.min(point.x());
                                    max_x = max_x.max(point.x());
                                    min_y = min_y.min(point.y());
                                    max_y = max_y.max(point.y());
                                    points.push((point.x(), point.y()));
                                }
                                polygons.push(points);
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }

    let root = BitMapBackend::new("japan_map.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("日本地図", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    for polygon in polygons {
        println!("drowing...");
        chart.draw_series(LineSeries::new(
            polygon.iter().map(|&(x, y)| (x, y)),
            &BLUE.mix(0.2),
        ))?;
    }

    println!("finished!");

    Ok(())
}
