async function get_distro_list() {
    const response = await fetch("/distro_list");
    const data = await response.json();
    return data;
}

async function get_distro_update_date(distro) {
    const response = await fetch("/date/" + distro);
    const data = await response.json();
    return data;
}

async function get_distro_update_status(distro) {
    let map = new Map();
    map["fail"] = "실패";
    map["success"] = "성공";
    map["mirroring"] = "진행 중...";

    const response = await fetch("/status/" + distro);
    const data = await response.json();
    return map[data];
}

async function display_distro() {
    let distro_list = await get_distro_list();

    // 모든 배포판에 대한 Promise를 생성합니다.
    let promises = distro_list.map(async distro => {
        let [update_date, update_status] = await Promise.all([
            get_distro_update_date(distro),
            get_distro_update_status(distro)
        ]);

        let color = "rgb(80, 244, 80)";
        if (update_status == "실패") {
            color = "rgb(254, 70, 70)";
        } else if (update_status == "진행 중...") {
            color = "rgb(94, 209, 247)";
        }

        let html = `
            <div class="distro" onclick="window.location.href='/${distro}'">
                <h2>${distro}</h2>
                <h3 style="color: ${color}">${update_status}</h3>
                <h3>${update_date}</h3>
            </div>
        `;
        
        return html;
    });

    let htmls = await Promise.all(promises);

    document.getElementsByClassName("mirror_list")[0].innerHTML = htmls.join('');
}