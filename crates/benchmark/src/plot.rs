use plotters::prelude::*;

use super::BenchmarkResult;

fn max<T>(v1: T, v2: T) -> T
where
    T: Ord + Eq,
{
    if v1 > v2 {
        v1
    } else {
        v2
    }
}

pub fn plot(
    nullable: &mut BenchmarkResult,
    crowbar: &mut BenchmarkResult,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("absc_crowbar.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut max_y = max(nullable.max_time().unwrap(), crowbar.max_time().unwrap()).as_millis();
    // Add padding
    max_y += max_y / 10;

    // We limit to 20, as Crowbar fails roughly at that point
    let max_x = max(
        max(nullable.max_num().unwrap(), crowbar.max_num().unwrap()),
        20,
    );

    let mut chart = ChartBuilder::on(&root)
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(1..max_x, 0..max_y)?;

    chart
        .configure_mesh()
        .x_desc("Number of Classes")
        .y_desc("Runtime in ms")
        .draw()?;

    chart
        .draw_series(LineSeries::new(nullable.to_points(), &RED))?
        .label("absc")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(crowbar.to_points(), &BLUE))?
        .label("Crowbar")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn plot_nullable(nullable: &mut BenchmarkResult) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("absc_only.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut max_y = nullable.max_time().unwrap().as_millis();
    max_y += max_y / 5;

    let mut min_y = nullable.min_time().unwrap().as_millis();
    min_y -= max_y / 5;

    println!("min_y: {}, max_y:{}", min_y, max_y);

    let max_x = nullable.max_num().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(1..max_x, min_y..max_y)?;

    chart
        .configure_mesh()
        .x_desc("Number of Classes")
        .y_desc("Runtime in ms")
        .draw()?;

    chart.draw_series(LineSeries::new(nullable.to_points(), &RED))?;

    /* chart
    .configure_series_labels()
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
    .draw()?; */

    Ok(())
}
