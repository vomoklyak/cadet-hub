use crate::CadetHubBeResult;
use common::model::{
    Cadet, CadetCourse, CadetCourseEntry, CadetCourseEntryBuilder, ImpexCadetCourseEntry, User,
    UserBuilder, UserRole,
};
use common::util::date_time_util;
use common::util::string_util::capitalize;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tempfile::{tempdir, TempDir};
use tokio::fs;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

// USERS
pub(crate) fn user(index: usize) -> User {
    UserBuilder::default()
        .id(index as i64)
        .login(format!("login_{index}"))
        .password(format!("hashed_password_{index}"))
        .role(UserRole::Reader)
        .build()
        .expect("failed build user")
}

pub(crate) fn actor_user_admin() -> User {
    UserBuilder::default()
        .id(Some(1))
        .login("admin")
        .password("password")
        .role(UserRole::Admin)
        .build()
        .expect("failed build user")
}

pub(crate) fn actor_user_writer() -> User {
    UserBuilder::default()
        .id(Some(1))
        .login("writer")
        .password("password")
        .role(UserRole::Writer)
        .build()
        .expect("failed build user")
}

pub(crate) fn actor_user_reader() -> User {
    UserBuilder::default()
        .id(Some(1))
        .login("reader")
        .password("password")
        .role(UserRole::Reader)
        .build()
        .expect("failed build user")
}

// CADETS
pub(crate) fn lowercase_cadet(id: i64, suffix: &str) -> Cadet {
    let mut cadet = cadet(id, suffix);
    cadet.set_tax_number(cadet.tax_number().to_lowercase());
    cadet.set_first_name(cadet.first_name().to_lowercase());
    cadet.set_middle_name(cadet.middle_name().to_lowercase());
    cadet.set_last_name(cadet.last_name().to_lowercase());
    cadet
}

pub(crate) fn cadet(id: i64, suffix: &str) -> Cadet {
    let mut cadet = Cadet::from(&ImpexCadetCourseEntry::from(&entry(id, suffix)));
    cadet.set_id(Some(id));
    cadet.set_last_name(capitalize(cadet.last_name()));
    cadet
}

// CADET COURSES
pub(crate) fn lowercase_cadet_course(id: i64, index: &str) -> CadetCourse {
    let mut cadet_course = cadet_course(id, index);
    cadet_course.set_military_rank(cadet_course.military_rank().to_lowercase());
    cadet_course.set_category(cadet_course.category().to_lowercase());
    cadet_course
}

pub(crate) fn cadet_course(id: i64, suffix: &str) -> CadetCourse {
    let mut cadet_course = CadetCourse::from(&ImpexCadetCourseEntry::from(&entry(id, suffix)));
    cadet_course.set_id(Some(id));
    cadet_course.set_cadet_id(Some(id));
    cadet_course
}

// CADET COURSE ENTRIES
pub(crate) fn impex_entry(id: i64) -> ImpexCadetCourseEntry {
    ImpexCadetCourseEntry::from(&entry(id, &id.to_string()))
}

pub(crate) fn entry(id: i64, suffix: &str) -> CadetCourseEntry {
    let birth_date_str = format!("0{}.01.2020", id + 1);
    let tax_number = generate_tax_number(&birth_date_str, id)
        .to_uppercase()
        .to_string();

    CadetCourseEntryBuilder::default()
        .first_name(format!("First_name_{suffix}"))
        .middle_name(format!("Middle_name_{suffix}"))
        .last_name(format!("Last_name_{suffix}"))
        .military_rank(format!("military_rank_{suffix}"))
        .birth_date(timestamp(&birth_date_str))
        .tax_number(tax_number)
        .source_unit(format!("source_unit_{suffix}"))
        .specialty_name(format!("specialty_name_{suffix}"))
        .specialty_code(format!("specialty_code_{suffix}"))
        .specialty_mos_code(format!("specialty_mos_code_{suffix}"))
        .category("CATEGORY")
        .training_location(format!("training_location_{suffix}"))
        .start_date(timestamp(&format!("01.02.201{id}")))
        .end_date(timestamp(&format!("01.02.202{id}")))
        .completion_order_number(format!("completion_order_number_{suffix}"))
        .completion_certificate_number(format!("completion_certificate_number_{suffix}"))
        .notes(format!("notes_{suffix}"))
        .build()
        .expect("failed build CadetCourseEntry")
}

pub(crate) fn generate_tax_number(birth_date_str: &str, index: i64) -> String {
    let days_since_base_tax_number_date =
        date_time_util::days_since_base_tax_number_date(birth_date_str)
            .expect("failed count days since base tax number date");
    let first_nine_digits = format!(
        "{:05}{:03}{}",
        days_since_base_tax_number_date,
        0,
        index % 10
    );
    let multipliers = [-1, 5, 7, 9, 4, 6, 10, 5, 7];
    let sum: i32 = first_nine_digits
        .chars()
        .enumerate()
        .map(|(i, char)| (char.to_digit(10).unwrap() as i32) * multipliers[i])
        .sum();
    let checksum = sum.rem_euclid(11).rem_euclid(10);
    format!("{}{}", first_nine_digits, checksum)
}

pub(crate) fn timestamp(date_str: &str) -> i64 {
    date_time_util::dot_dd_mm_yyyy_str_as_utc_timestamp(date_str).expect("failed parse timestamp")
}

// FILES
pub(crate) async fn create_file_if_not_exist(path_str: &str) -> String {
    let path = Path::new(path_str);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .await
            .expect("failed create path");
    }
    path_str.to_string()
}
pub(crate) async fn file_content(path_str: &str) -> Cow<'static, [u8]> {
    fs::read(path_str).await.expect("failed read file").into()
}

pub(crate) async fn temp_dir() -> CadetHubBeResult<TempDir> {
    let temp_dir = tempdir()?;
    Ok(temp_dir)
}

pub(crate) async fn temp_file(
    temp_dir: &TempDir,
    file_name: &str,
    file_content: &str,
) -> CadetHubBeResult<File> {
    let file_path = temp_dir.path().join(file_name);
    let mut file = File::create(file_path)?;
    file.write_all(file_content.as_bytes())?;
    Ok(file)
}

pub(crate) async fn zip_file_content(zip_path: &str, file_name: &str) -> CadetHubBeResult<String> {
    let mut zip_file = ZipArchive::new(File::open(zip_path)?)?;
    let mut file = zip_file.by_name(file_name)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub(crate) async fn zip(
    zip_path: &str,
    file_name_to_file_content: Vec<(String, String)>,
) -> CadetHubBeResult<()> {
    let options: FileOptions<'_, ()> =
        FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let mut zip_file = ZipWriter::new(File::create(zip_path)?);
    for (file_name, file_content) in file_name_to_file_content {
        zip_file.start_file(file_name, options)?;
        zip_file.write_all(file_content.as_bytes())?;
    }
    zip_file.finish()?;
    Ok(())
}