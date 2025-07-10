use std::{io::Cursor, ops::Add, sync::Arc};

use axum::{body::Body, http::{header, Response}, response::IntoResponse, Extension};
use genpdf::{elements::{Break, LinearLayout, Paragraph, TableLayout}, style, Alignment, Document, Element, Margins, Position, Size};
use num_format::{Locale, ToFormattedString};
use services::{db_connection::DB, redis::Redis, response::reports::{ResponseSummaryImplementationByYear, ResponseSummaryImplementationByYearFullOverview, ResponseSummaryImplementationByYearOverview}};

use crate::helpers::{app_icon_images, city_images, doc_title, font_family};


pub async fn reports_summary_implementation_by_year (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {
    let mut redis = Redis::new("summary_implementation_by_year".to_string(), db.redis_connection.clone());

    let sse = Arc::new(db.sse_connection.clone());    

    let get_redis_value = redis.get_value().await;

    if get_redis_value.is_none() {
        // SSE EVENTS
        let clone_sse = sse.clone();
        tokio::spawn(async move {
            clone_sse.send_progress_update("projects-summary-implementation", 0, "No Data Found");
        });

        return Response::default();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-implementation", 0, "Preparing data information");
    });

    let get_parse_projects: ResponseSummaryImplementationByYearFullOverview = serde_json::from_str(&get_redis_value.unwrap()).unwrap();

    let get_total_projects = get_parse_projects.total_projects;
    let get_projects = get_parse_projects.projects_data;

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-implementation", 1, "Preparing report document");
    });

    let mut doc = Document::new(font_family());
    doc.set_title("City Engineer's Office - Management Information System");
    doc.set_paper_size(Size::new(297.0, 210.0));


    doc.push(city_images().with_position(Position::new(40, 10)));
    doc.push(app_icon_images().with_position(Position::new(25, 10)));
    doc.push(doc_title());

    // Project Information
    let reports_title = Paragraph::new("Summary of project implementation")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(17))
        .padded(Margins::trbl(5, 0, 0, 0));

    let mut get_project_year: Vec<i32> = get_projects.iter().map(|v| v.year.to_owned()).collect();
    get_project_year.sort();

    // Projects Summary Table
    let mut table_projects_summary = TableLayout::new(vec![1; get_projects.len().add(2)]);

    // ROW HEAD
    {
        let mut row_project_summary_head = table_projects_summary.row();

        row_project_summary_head.push_element(Paragraph::new("Status of Project Implementation")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)).padded(Margins::trbl(0, 0, 0, 2)));

        for year in get_project_year.clone() {
             row_project_summary_head.push_element(Paragraph::new(year.to_string())
                .aligned(Alignment::Left)
                .styled(style::Effect::Bold)
                .styled(style::Style::new().with_font_size(7)));
        }

        row_project_summary_head.push_element(Paragraph::new("Rate Percentage")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push().unwrap();
    }

    // TABLE DATA FOR NOT YET STARTED
    {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new("Not Yet Started")
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        for year in get_project_year.clone() {
            
            let get_project_by_year: Vec<ResponseSummaryImplementationByYearOverview> = get_projects.iter().filter_map(|v| {
                if v.year == year {
                    Some(v.to_owned())
                } else {
                    None
                }
            }).collect();

            let parse_project_by_year = get_project_by_year.iter().next().unwrap();

            let get_projects_by_status = parse_project_by_year.data.iter().find(|v| v.project_status == "Not Yet Started").cloned().unwrap_or_default();

            let mut main_layout = LinearLayout::vertical();

            let mut projects_layout = LinearLayout::vertical();
            projects_layout.push(Paragraph::new("Projects")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            projects_layout.push(Paragraph::new(get_projects_by_status.total_projects.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            let mut appropriation_layout = LinearLayout::vertical();
            appropriation_layout.push(Paragraph::new("Appropriation")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            appropriation_layout.push(Paragraph::new(get_projects_by_status.total_appropriation.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            main_layout.push(projects_layout);
            main_layout.push(appropriation_layout);


            row_project_summary_data.push_element(main_layout);
        }

        let parse_project_by_not_yet_started: Vec<ResponseSummaryImplementationByYear> = get_projects.iter().filter_map(|v| {
                let get_project_by_status = v.data.iter().find(|v| v.project_status == "Not Yet Started").cloned().unwrap_or_default();

                Some(get_project_by_status)
        }).collect();

        let total_projects = parse_project_by_not_yet_started.iter().fold(0, |acc, n| acc + n.total_projects);

        let make_average = (total_projects as f64 / get_total_projects as f64) * 100.0;

        row_project_summary_data.push_element(Paragraph::new(format!("{make_average:.2} %"))
            .padded(Margins::trbl(1, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        row_project_summary_data.push().unwrap();
    }

    // TABLE DATA FOR PREPARATION
    {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new("Preparation")
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        for year in get_project_year.clone() {
            
            let get_project_by_year: Vec<ResponseSummaryImplementationByYearOverview> = get_projects.iter().filter_map(|v| {
                if v.year == year {
                    Some(v.to_owned())
                } else {
                    None
                }
            }).collect();

            let parse_project_by_year = get_project_by_year.iter().next().unwrap();

            let get_projects_by_status = parse_project_by_year.data.iter().find(|v| v.project_status == "Preparation").cloned().unwrap_or_default();

            let mut main_layout = LinearLayout::vertical();

            let mut projects_layout = LinearLayout::vertical();
            projects_layout.push(Paragraph::new("Projects")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            projects_layout.push(Paragraph::new(get_projects_by_status.total_projects.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            let mut appropriation_layout = LinearLayout::vertical();
            appropriation_layout.push(Paragraph::new("Appropriation")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            appropriation_layout.push(Paragraph::new(get_projects_by_status.total_appropriation.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            main_layout.push(projects_layout);
            main_layout.push(appropriation_layout);


            row_project_summary_data.push_element(main_layout);
        }

        let parse_project_by_not_yet_started: Vec<ResponseSummaryImplementationByYear> = get_projects.iter().filter_map(|v| {
                let get_project_by_status = v.data.iter().find(|v| v.project_status == "Preparation").cloned().unwrap_or_default();

                Some(get_project_by_status)
        }).collect();

        let total_projects = parse_project_by_not_yet_started.iter().fold(0, |acc, n| acc + n.total_projects);

        let make_average = (total_projects as f64 / get_total_projects as f64) * 100.0;

        row_project_summary_data.push_element(Paragraph::new(format!("{make_average:.2} %"))
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        row_project_summary_data.push().unwrap();
    }

    // TABLE DATA FOR BIDDING
    {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new("Bidding")
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        for year in get_project_year.clone() {
            
            let get_project_by_year: Vec<ResponseSummaryImplementationByYearOverview> = get_projects.iter().filter_map(|v| {
                if v.year == year {
                    Some(v.to_owned())
                } else {
                    None
                }
            }).collect();

            let parse_project_by_year = get_project_by_year.iter().next().unwrap();

            let get_projects_by_status = parse_project_by_year.data.iter().find(|v| v.project_status == "Bidding").cloned().unwrap_or_default();

            let mut main_layout = LinearLayout::vertical();

            let mut projects_layout = LinearLayout::vertical();
            projects_layout.push(Paragraph::new("Projects")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            projects_layout.push(Paragraph::new(get_projects_by_status.total_projects.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            let mut appropriation_layout = LinearLayout::vertical();
            appropriation_layout.push(Paragraph::new("Appropriation")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            appropriation_layout.push(Paragraph::new(get_projects_by_status.total_appropriation.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            main_layout.push(projects_layout);
            main_layout.push(appropriation_layout);


            row_project_summary_data.push_element(main_layout);
        }

        let parse_project_by_not_yet_started: Vec<ResponseSummaryImplementationByYear> = get_projects.iter().filter_map(|v| {
                let get_project_by_status = v.data.iter().find(|v| v.project_status == "Bidding").cloned().unwrap_or_default();

                Some(get_project_by_status)
        }).collect();

        let total_projects = parse_project_by_not_yet_started.iter().fold(0, |acc, n| acc + n.total_projects);

        let make_average = (total_projects as f64 / get_total_projects as f64) * 100.0;

        row_project_summary_data.push_element(Paragraph::new(format!("{make_average:.2} %"))
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        row_project_summary_data.push().unwrap();
    }

    // TABLE DATA FOR BIDDED
    {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new("Bidded")
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        for year in get_project_year.clone() {
            
            let get_project_by_year: Vec<ResponseSummaryImplementationByYearOverview> = get_projects.iter().filter_map(|v| {
                if v.year == year {
                    Some(v.to_owned())
                } else {
                    None
                }
            }).collect();

            let parse_project_by_year = get_project_by_year.iter().next().unwrap();

            let get_projects_by_status = parse_project_by_year.data.iter().find(|v| v.project_status == "Bidded").cloned().unwrap_or_default();

            let mut main_layout = LinearLayout::vertical();

            let mut projects_layout = LinearLayout::vertical();
            projects_layout.push(Paragraph::new("Projects")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            projects_layout.push(Paragraph::new(get_projects_by_status.total_projects.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            let mut appropriation_layout = LinearLayout::vertical();
            appropriation_layout.push(Paragraph::new("Appropriation")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            appropriation_layout.push(Paragraph::new(get_projects_by_status.total_appropriation.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            main_layout.push(projects_layout);
            main_layout.push(appropriation_layout);


            row_project_summary_data.push_element(main_layout);
        }

        let parse_project_by_not_yet_started: Vec<ResponseSummaryImplementationByYear> = get_projects.iter().filter_map(|v| {
                let get_project_by_status = v.data.iter().find(|v| v.project_status == "Bidded").cloned().unwrap_or_default();

                Some(get_project_by_status)
        }).collect();

        let total_projects = parse_project_by_not_yet_started.iter().fold(0, |acc, n| acc + n.total_projects);

        let make_average = (total_projects as f64 / get_total_projects as f64) * 100.0;

        row_project_summary_data.push_element(Paragraph::new(format!("{make_average:.2} %"))
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        row_project_summary_data.push().unwrap();
    }

    // TABLE DATA FOR ON-GOING
    {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new("On-Going")
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        for year in get_project_year.clone() {
            
            let get_project_by_year: Vec<ResponseSummaryImplementationByYearOverview> = get_projects.iter().filter_map(|v| {
                if v.year == year {
                    Some(v.to_owned())
                } else {
                    None
                }
            }).collect();

            let parse_project_by_year = get_project_by_year.iter().next().unwrap();

            let get_projects_by_status = parse_project_by_year.data.iter().find(|v| v.project_status == "On-Going").cloned().unwrap_or_default();

            let mut main_layout = LinearLayout::vertical();

            let mut projects_layout = LinearLayout::vertical();
            projects_layout.push(Paragraph::new("Projects")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            projects_layout.push(Paragraph::new(get_projects_by_status.total_projects.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            let mut appropriation_layout = LinearLayout::vertical();
            appropriation_layout.push(Paragraph::new("Appropriation")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            appropriation_layout.push(Paragraph::new(get_projects_by_status.total_appropriation.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            main_layout.push(projects_layout);
            main_layout.push(appropriation_layout);


            row_project_summary_data.push_element(main_layout);
        }

        let parse_project_by_not_yet_started: Vec<ResponseSummaryImplementationByYear> = get_projects.iter().filter_map(|v| {
                let get_project_by_status = v.data.iter().find(|v| v.project_status == "On-Going").cloned().unwrap_or_default();

                Some(get_project_by_status)
        }).collect();

        let total_projects = parse_project_by_not_yet_started.iter().fold(0, |acc, n| acc + n.total_projects);

        let make_average = (total_projects as f64 / get_total_projects as f64) * 100.0;

        row_project_summary_data.push_element(Paragraph::new(format!("{make_average:.2} %"))
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        row_project_summary_data.push().unwrap();
    }

    // TABLE DATA FOR COMPLETED
    {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new("Completed")
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        for year in get_project_year.clone() {
            
            let get_project_by_year: Vec<ResponseSummaryImplementationByYearOverview> = get_projects.iter().filter_map(|v| {
                if v.year == year {
                    Some(v.to_owned())
                } else {
                    None
                }
            }).collect();

            let parse_project_by_year = get_project_by_year.iter().next().unwrap();

            let get_projects_by_status = parse_project_by_year.data.iter().find(|v| v.project_status == "Completed").cloned().unwrap_or_default();

            let mut main_layout = LinearLayout::vertical();

            let mut projects_layout = LinearLayout::vertical();
            projects_layout.push(Paragraph::new("Projects")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            projects_layout.push(Paragraph::new(get_projects_by_status.total_projects.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            let mut appropriation_layout = LinearLayout::vertical();
            appropriation_layout.push(Paragraph::new("Appropriation")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            appropriation_layout.push(Paragraph::new(get_projects_by_status.total_appropriation.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            main_layout.push(projects_layout);
            main_layout.push(appropriation_layout);


            row_project_summary_data.push_element(main_layout);
        }

        let parse_project_by_not_yet_started: Vec<ResponseSummaryImplementationByYear> = get_projects.iter().filter_map(|v| {
                let get_project_by_status = v.data.iter().find(|v| v.project_status == "Completed").cloned().unwrap_or_default();

                Some(get_project_by_status)
        }).collect();

        let total_projects = parse_project_by_not_yet_started.iter().fold(0, |acc, n| acc + n.total_projects);

        let make_average = (total_projects as f64 / get_total_projects as f64) * 100.0;

        row_project_summary_data.push_element(Paragraph::new(format!("{make_average:.2} %"))
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        row_project_summary_data.push().unwrap();
    }

    // TABLE DATA FOR SUSPENDED
    {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new("Suspended")
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        for year in get_project_year.clone() {
            
            let get_project_by_year: Vec<ResponseSummaryImplementationByYearOverview> = get_projects.iter().filter_map(|v| {
                if v.year == year {
                    Some(v.to_owned())
                } else {
                    None
                }
            }).collect();

            let parse_project_by_year = get_project_by_year.iter().next().unwrap();

            let get_projects_by_status = parse_project_by_year.data.iter().find(|v| v.project_status == "Suspended").cloned().unwrap_or_default();

            let mut main_layout = LinearLayout::vertical();

            let mut projects_layout = LinearLayout::vertical();
            projects_layout.push(Paragraph::new("Projects")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            projects_layout.push(Paragraph::new(get_projects_by_status.total_projects.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            let mut appropriation_layout = LinearLayout::vertical();
            appropriation_layout.push(Paragraph::new("Appropriation")
                .styled(style::Style::new().with_font_size(7))
                .padded(Margins::trbl(2, 0, 0, 0))
            );
            appropriation_layout.push(Paragraph::new(get_projects_by_status.total_appropriation.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

            main_layout.push(projects_layout);
            main_layout.push(appropriation_layout);


            row_project_summary_data.push_element(main_layout);
        }

        let parse_project_by_not_yet_started: Vec<ResponseSummaryImplementationByYear> = get_projects.iter().filter_map(|v| {
                let get_project_by_status = v.data.iter().find(|v| v.project_status == "Suspended").cloned().unwrap_or_default();

                Some(get_project_by_status)
        }).collect();

        let total_projects = parse_project_by_not_yet_started.iter().fold(0, |acc, n| acc + n.total_projects);

        let make_average = (total_projects as f64 / get_total_projects as f64) * 100.0;

        row_project_summary_data.push_element(Paragraph::new(format!("{make_average:.2} %"))
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );

        row_project_summary_data.push().unwrap();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-implementation", 2, "Compiling project summary informations");
    });

    doc.push(reports_title); 
    doc.push(Break::new(1));
    doc.push(table_projects_summary);

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-implementation", 3, "Generating PDF file");
    });

    let mut buffer = Cursor::new(Vec::new());
    doc.render(&mut buffer).expect("Failed to render PDF Document");

    let pdf_bytes = buffer.into_inner();

    let projects_file_name = format!("projects-implementation-by-year.pdf");
    let attachment = format!("attachment; filename=\"{}\"", projects_file_name);

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-implementation", 4, "PDF Report Completed");
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, attachment)
        .header(header::CONTENT_LENGTH, pdf_bytes.len())
        .body(Body::from(pdf_bytes))
        .unwrap()
}