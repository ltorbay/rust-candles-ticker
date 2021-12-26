use std::ops::Div;

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use plotters::prelude::{BitMapBackend, CandleStick, ChartBuilder, GREEN, IntoDrawingArea, IntoFont, RED, WHITE};
use plotters_canvas::CanvasBackend;

use crate::model::crypto_compare::Histohour;

const Y_SCALE_FACTOR: f32 = 0.1;

// TODO map error to internal
pub fn plot(histogram: &Histohour) -> Result<(), Box<dyn std::error::Error>> {
    let canvas = CanvasBackend::new("canvas_id").expect("cannot find canvas")
        .into_drawing_area();

    let root = BitMapBackend::new("plotters-data/candles.png", (1024, 768))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let (from_date, to_date) = (
        // TODO Add the duration into an enum typed histogram?
        parse_time(histogram.data.time_from) - Duration::hours(1).div(2),
        parse_time(histogram.data.time_to) + Duration::hours(1).div(2)
    );

    let (min, max) = min_max(histogram);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .caption("Candles", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, min..max)?;

    chart.configure_mesh().light_line_style(&WHITE).draw()?;

    chart.draw_series(
        histogram.data.data.iter()
            .map(|x| CandleStick::new(parse_time(x.time),
                                      x.open,
                                      x.high,
                                      x.low,
                                      x.close,
                                      &GREEN,
                                      &RED,
                                      77)),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-data' dir exists under current dir");
    Ok(())
}

fn min_max(histogram: &Histohour) -> (f32, f32) {
    let (min, max) = (histogram.data.data.iter().map(|x| x.low).fold(f32::INFINITY, f32::min),
                      histogram.data.data.iter().map(|x| x.high).fold(f32::NEG_INFINITY, f32::max));
    let scale_corrector = (max - min) * Y_SCALE_FACTOR;
    let (min, max) = (min - scale_corrector, max + scale_corrector);
    (min.round(), max.round())
}

fn parse_time(t: i64) -> DateTime<Utc> {
    DateTime::from_utc(NaiveDateTime::from_timestamp(t, 0), Utc)
}

#[cfg(test)]
mod tests {
    use crate::model::crypto_compare::{Histohour, Histogram, HistoData};
    use assert_approx_eq::assert_approx_eq;


    #[test]
    fn limits_are_found() {
        let histogram = Histohour {
            response: "".to_string(),
            has_warning: false,
            data: Histogram {
                time_from: 0,
                time_to: 0,
                data: vec![
                    HistoData {
                        time: 0,
                        high: 12.0,
                        low: 1.0,
                        open: 0.0,
                        close: 0.0,
                        conversion_type: "".to_string(),
                        conversion_symbol: "".to_string(),
                    },
                    HistoData {
                        time: 0,
                        high: 25.0,
                        low: 22.0,
                        open: 0.0,
                        close: 0.0,
                        conversion_type: "".to_string(),
                        conversion_symbol: "".to_string(),
                    },
                    HistoData {
                        time: 0,
                        high: 24.0,
                        low: 3.0,
                        open: 0.0,
                        close: 0.0,
                        conversion_type: "".to_string(),
                        conversion_symbol: "".to_string(),
                    }
                ],
            },
        };
        
        let (min, max) = super::min_max(&histogram);
        assert_approx_eq!(min, -1f32);
        assert_approx_eq!(max, 27f32);
    }
}

