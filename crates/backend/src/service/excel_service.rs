use crate::CadetHubBeResult;
use common::error::CadetHubError;
use common::logger::warn;
use common::model::{
    ExcelTemplate, ExcelTemplateBuilder, ExcelTemplateName, ImpexCadetCourseEntry,
    ImpexCadetCourseEntryBuilder, ReadExcelFileRequest, WriteExcelFileRequest,
};
use common::util::date_time_util;
use common::util::date_time_util::{dot_dd_mm_yyyy_str_as_utc_timestamp, SECONDS_IN_DAY};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use umya_spreadsheet::reader;
use umya_spreadsheet::writer;
use umya_spreadsheet::Worksheet;

pub(crate) struct ExcelService {
    template_registry: ExcelTemplateRegistry,
}

impl ExcelService {
    pub(crate) fn new() -> Self {
        Self {
            template_registry: ExcelTemplateRegistry::new(),
        }
    }

    pub(crate) async fn get_excel_template(&self, name: ExcelTemplateName) -> &ExcelTemplate {
        self.template_registry.require_template(name)
    }

    pub(crate) fn read_impex_cadet_course_entries(
        &self,
        request: ReadExcelFileRequest,
    ) -> CadetHubBeResult<Vec<ImpexCadetCourseEntry>> {
        let mut entries = vec![];
        let template = self
            .template_registry
            .require_template(ExcelTemplateName::CadetCourseReport);
        let sheet_number = request
            .sheet_number()
            .unwrap_or(template.sheet_number().clone());
        let first_column = request
            .first_column()
            .unwrap_or(template.first_column().clone());
        let first_row = request.first_row().unwrap_or(template.first_row().clone());

        let mut book =
            reader::xlsx::read(request.path()).map_err(CadetHubError::general_error_with_source)?;
        let Some(sheet) = book.get_sheet_mut(&(sheet_number as usize)) else {
            warn!(
                "Excel file does not contain sheet: path={:?}, sheet_number={:?}",
                request.path(),
                sheet_number
            );
            return Ok(entries);
        };

        let skip_optional_validation = request.skip_optional_validation().clone();
        let column = |index: u32| index + first_column;
        let last_row = request
            .last_row()
            .clone()
            .unwrap_or(sheet.get_highest_row());
        for current_row in first_row..=last_row {
            if ExcelUtil::cell_is_empty(sheet, first_column, current_row) {
                break;
            }
            let mut entry = ImpexCadetCourseEntryBuilder::default()
                .military_rank(ExcelUtil::cell_as_string(sheet, column(1), current_row))
                .full_name(ExcelUtil::cell_as_string(sheet, column(2), current_row))
                .birth_date(ExcelUtil::cell_as_date_string(
                    sheet,
                    column(3),
                    current_row,
                ))
                .tax_number(ExcelUtil::cell_as_string(sheet, column(4), current_row))
                .source_unit(ExcelUtil::cell_as_string(sheet, column(5), current_row))
                .specialty_name(ExcelUtil::cell_as_string(sheet, column(6), current_row))
                .specialty_code(ExcelUtil::cell_as_string(sheet, column(7), current_row))
                .specialty_mos_code(ExcelUtil::cell_as_string(sheet, column(8), current_row))
                .category(ExcelUtil::cell_as_string(sheet, column(9), current_row))
                .training_location(ExcelUtil::cell_as_string(sheet, column(10), current_row))
                .start_date(ExcelUtil::cell_as_date_string(
                    sheet,
                    column(11),
                    current_row,
                ))
                .end_date(ExcelUtil::cell_as_date_string(
                    sheet,
                    column(12),
                    current_row,
                ))
                .completion_order_number(ExcelUtil::cell_as_string(sheet, column(13), current_row))
                .completion_certificate_number(ExcelUtil::cell_as_string(
                    sheet,
                    column(14),
                    current_row,
                ))
                .notes(ExcelUtil::cell_as_string(sheet, column(15), current_row))
                .build()
                .expect("failed build ImpexCadetCourseEntryBuilder");
            if skip_optional_validation {
                entry.set_skip_optional_validation(Some(true));
            }
            entries.push(entry)
        }
        Ok(entries)
    }

    pub(crate) fn write_impex_cadet_course_entries(
        &self,
        request: WriteExcelFileRequest,
        entries: Vec<ImpexCadetCourseEntry>,
    ) -> CadetHubBeResult<()> {
        let template = self
            .template_registry
            .require_template(ExcelTemplateName::CadetCourseReport);
        let mut book = reader::xlsx::read_reader(Cursor::new(template.data()), true)
            .map_err(CadetHubError::general_error_with_source)?;
        let Some(sheet) = book.get_sheet_mut(&(template.sheet_number().clone() as usize)) else {
            return Ok(());
        };

        let column = |index: u32| index + template.first_column();
        for (row_index, entry) in entries.iter().enumerate() {
            let current_row = template.first_row() + (row_index as u32);
            sheet.copy_row_styling(&8, &current_row, None, None);
            sheet
                .get_cell_mut((column(0), current_row))
                .set_value((row_index + 1).to_string());
            sheet
                .get_cell_mut((column(1), current_row))
                .set_value(entry.military_rank().clone());
            sheet
                .get_cell_mut((column(2), current_row))
                .set_value(entry.full_name().clone());
            Self::set_date_cell(sheet, column(3), current_row, entry.birth_date());
            sheet
                .get_cell_mut((column(4), current_row))
                .set_value(entry.tax_number().clone());
            sheet
                .get_cell_mut((column(5), current_row))
                .set_value(entry.source_unit().clone());
            sheet
                .get_cell_mut((column(6), current_row))
                .set_value(entry.specialty_name().clone());
            sheet
                .get_cell_mut((column(7), current_row))
                .set_value(entry.specialty_code().clone());
            sheet
                .get_cell_mut((column(8), current_row))
                .set_value(entry.specialty_mos_code().clone());
            sheet
                .get_cell_mut((column(9), current_row))
                .set_value(entry.category().clone());
            sheet
                .get_cell_mut((column(10), current_row))
                .set_value(entry.training_location().clone());
            Self::set_date_cell(sheet, column(11), current_row, entry.start_date());
            Self::set_date_cell(sheet, column(12), current_row, entry.end_date());
            sheet
                .get_cell_mut((column(13), current_row))
                .set_value(entry.completion_order_number().clone());
            sheet
                .get_cell_mut((column(14), current_row))
                .set_value(entry.completion_certificate_number().clone());
            if let Some(notes) = &entry.notes() {
                sheet
                    .get_cell_mut((column(15), current_row))
                    .set_value(notes.clone());
            }
            if let Some(error) = &entry.error() {
                sheet
                    .get_cell_mut((column(16), current_row))
                    .set_value(error.clone());
            }
        }
        writer::xlsx::write(&book, Path::new(request.path()))
            .map_err(|error| CadetHubError::general_error_with_source(error).into())
    }

    fn set_date_cell(sheet: &mut Worksheet, column: u32, row: u32, date: &str) {
        let cell = sheet.get_cell_mut((column, row));
        if let Ok(excel_date) = ExcelUtil::dot_dd_mm_yyyy_date_str_to_excel_date(date) {
            cell.set_value_number(excel_date);
        } else {
            cell.set_value(date);
        }
    }
}

struct ExcelTemplateRegistry {
    name_to_template: HashMap<ExcelTemplateName, ExcelTemplate>,
}

impl ExcelTemplateRegistry {
    fn new() -> Self {
        let cadet_course_template = ExcelTemplateBuilder::default()
            .sheet_number(0u32)
            .first_row(8u32)
            .first_column(1u32)
            .data(include_bytes!("../resources/templates/cadet_course_report.xlsx"))
            .build()
            .expect("failed build ExcelTemplate");
        let type_to_template =
            HashMap::from([(ExcelTemplateName::CadetCourseReport, cadet_course_template)]);
        Self {
            name_to_template: type_to_template,
        }
    }

    fn require_template(&self, template_type: ExcelTemplateName) -> &ExcelTemplate {
        self.name_to_template
            .get(&template_type)
            .expect("failed get ExcelTemplate")
    }
}

struct ExcelUtil {}

impl ExcelUtil {
    // days between Excel Zero Date 12/30/1899 and Unix Epoch 01/01/1970
    const UNIX_EPOCH_OFFSET: f64 = 25569.0;

    fn cell_as_string(sheet: &mut Worksheet, column: u32, row: u32) -> String {
        sheet.get_value((column, row))
    }

    fn cell_as_date_string(sheet: &mut Worksheet, column: u32, row: u32) -> String {
        if let Some(excel_date) = sheet.get_value_number((column, row)) {
            let unix_timestamp = Self::excel_date_to_unix_timestamp(excel_date);
            date_time_util::utc_timestamp_as_dot_dd_mm_yyyy_str(unix_timestamp).unwrap()
        } else {
            sheet.get_value((column, row))
        }
    }

    fn cell_is_empty(sheet: &mut Worksheet, column: u32, row: u32) -> bool {
        sheet.get_value((column, row)).is_empty()
    }

    fn excel_date_to_unix_timestamp(excel_date: f64) -> i64 {
        ((excel_date - Self::UNIX_EPOCH_OFFSET) * SECONDS_IN_DAY as f64).round() as i64
    }

    fn dot_dd_mm_yyyy_date_str_to_excel_date(date_str: &str) -> CadetHubBeResult<f64> {
        let unix_timestamp = dot_dd_mm_yyyy_str_as_utc_timestamp(date_str)?;
        Ok(Self::unix_timestamp_to_excel_date(unix_timestamp))
    }

    fn unix_timestamp_to_excel_date(timestamp: i64) -> f64 {
        (timestamp as f64 / SECONDS_IN_DAY as f64) + Self::UNIX_EPOCH_OFFSET
    }
}
