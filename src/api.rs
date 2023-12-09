use std::{collections::HashMap, fs::File};
use std::io::Read;
use actix_web::{web, get, HttpResponse};

#[get("/distro_list")]
async fn get_distro_list() -> HttpResponse {
    HttpResponse::Ok().json(vec![
        "ubuntu",
        "ubuntu-cd",
        "archlinux"
    ])
}

#[get("/date/{distro}")]
async fn get_distro_update_date(distro: web::Path<String>) -> HttpResponse {
    let path = "/apt-mirror/mirror/status/";

    let distro_hm = HashMap::from([
        ("ubuntu", format!("{}{}", path, "ubuntu")),
        ("ubuntu-cd", format!("{}{}", path, "ubuntu_cd")),
        ("archlinux", format!("{}{}", path, "archlinux"))
    ]);
    
    let filename = distro_hm.get(distro.as_str());
    if filename.is_some() {
        let file_path = filename.unwrap().to_owned() + "-date";
        let mut f = File::open(&file_path).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Something went wrong reading the file");

        return HttpResponse::Ok().json(contents);
    }

    HttpResponse::NotFound().json("Not found")
}

#[get("/status/{distro}")]
async fn get_distro_status(distro: web::Path<String>) -> HttpResponse {
    // /apt-mirror/mirror/status/
    let path = "/apt-mirror/mirror/status/";

    let distro_hm = HashMap::from([
        ("ubuntu", format!("{}{}", path, "ubuntu")),
        ("ubuntu-cd", format!("{}{}", path, "ubuntu_cd")),
        ("archlinux", format!("{}{}", path, "archlinux"))
    ]);
    
    let filename = distro_hm.get(distro.as_str());
    if filename.is_some() {
        let mut f = File::open(filename.unwrap()).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Something went wrong reading the file");

        return HttpResponse::Ok().json(contents);
    }

    HttpResponse::NotFound().json("Not found")
}