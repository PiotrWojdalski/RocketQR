extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

// If we wanted or needed to serve files manually, we'd use `NamedFile`. Always
// prefer to use `FileServer`!
// mod manual {
//     use std::path::{PathBuf, Path};
//     use rocket::fs::NamedFile;

//     #[rocket::get("/second/<path..>")]
//     pub async fn second(path: PathBuf) -> Option<NamedFile> {
//         let mut path = Path::new(super::relative!("static")).join(path);
//         if path.is_dir() {
//             path.push("index.html");
//         }

//         NamedFile::open(path).await.ok()
//     }
// }

#[rocket::get("/")]
fn index() -> Template {
    Template::render("index", context! {
        job_description: "Strona została stworzona aby przybliżyć wszystkie zespoły w SSC"
    })
}

#[rocket::get("/it/index")]
fn it() -> Template {
    Template::render("it", context! {
        job_description: "IT"
    })
}

#[rocket::get("/md/index")]
fn md() -> Template {
    Template::render("md", context! {
        job_description: "Master data"
    })
}
#[rocket::get("/ac/index")]
fn ac() -> Template {
    Template::render("ac", context! {
        job_description: "Accounting"
    })
}

#[rocket::get("/it/eco")]
fn eco() -> Template {
    Template::render("it", context! {
        job_description: "Zarządzanie i konfiguracja Linux’a (RedHat ,CentOS).
		Monitorowanie i ostrzeganie przy pomocy Grafany, Prometheus’a oraz Zabbix’a.
		Zarządzanie i konfiguracja Vikingiem.
		Testy obciążeniowe (tworzenie scenariuszy w JavaScript, zbieranie wyników, prezentowanie wyników w Grafanie).
		Tworzenie dokumentacji.
		Projekt
		POS Recovery i Emergency POS.
		Programowanie w Python.
		Zarządzanie platformą Google."
    })
}
#[rocket::get("/it/application")]
fn application() -> Template {
    Template::render("it", context! {
        job_description: "Aplication support"
    })
}
#[rocket::get("/it/bi")]
fn bi() -> Template {
    Template::render("it", context! {
        job_description: "BI Support oferuje wsparcie użytkownikom wzakresie rozwiązań dostarczanych przez Business Intelligence ( BI pozwala zaprezentować dane w przyjazny sposób i ułatwić analizę biznesową Z perspektywy użytkownika końcowego Business Intelligence kojarzony jest
            przede wszystkim z otrzymywanymi na skrzynkę
            mailową raportami KPI, Analysis for Office i SAP
            Analytics Cloud"
    })
}
#[rocket::get("/it/desktop")]
fn desktop() -> Template {
    Template::render("it", context! {
        job_description: "Tworzenie, utrzymanie i obsługa obrazów stacji roboczych, w tym systemu operacyjnego, sterowników i pakietów oprogramowania wykorzystywanego organizacji. Wsparcie i obsługa systemu zarządzania ThinClient i PDA. Testowanie i udostępnianie poprawek bezpieczeństwa systemu operacyjnego. Zapewnienie wsparcia drugiego poziomu dla obsługiwanego oprogramowania komputerowego."
    })
}
#[rocket::get("/it/sd")]
fn sd() -> Template {
    Template::render("it", context! {
        job_description: "Zespół zapewnia pierwszą linię wsparcia i na co dzień kontaktuje się bezpośrednio z pracownikami różnych formatów i jednostek skupionych w Salling Group. Główne zadania to rozwiązywanie bieżących problemów technicznych związanych z działaniem rozległej sieci systemów informatycznych (SAP, CISCO, Windows, Linux).
         Dodatkowo zespół odpowiada za ewidencjonowanie błędów systemowych w oparciu o ramy czasowe, wspieranie duńskich zespołów 2nd level, pośredni i bezpośredni kontakt z użytkownikiem oraz tworzenie i edycję dokumentacji technicznej.
        "
    })
}
#[rocket::get("/it/mon")]
fn mon() -> Template {
    Template::render("it", context! {
        job_description: "Monitoring ogląda YT."
    })
}
#[rocket::get("/it/npl")]
fn npl() -> Template {
    Template::render("it", context! {
        job_description: "NPL to pierwsza linia wsparcia dla organizacji Netto PL Wspieramy sklepy, magazyny i HQ Mamy drugą linię wsparcia w Motańcu i Warszawie oraz 3 cią linię wsparcia w Motańcu."
    })
}
#[rocket::get("/it/change")]
fn change() -> Template {
    Template::render("it", context! {
        job_description: "Obsługa (przyjmowanie i zatwierdzanie) zmian w systemach produkcyjnych i nieprodukcyjnych (SAP, Non-SAP, Network, E-Commerce).Planowanie i informowanie organizacji o nadchodzących zmianach, freeze periods, maintenances etc.Tworzenie i aktualizacja Release Calendar.Prowadzenie Weekly CAB Meeting."
    })
}
#[rocket::get("/it/cctv")]
fn cctv() -> Template {
    Template::render("it", context! {
        job_description: "Obsługa i monitorowanie systemu CCTV w celu oceny sytuacji wysokiego ryzyka. Reagowanie, gdy zostanie wykryta sytuacja oszustwa. Udział w określaniu i rozwiązywaniu problemów w oparciu o istniejące procedury. Kalibracja systemu nadzoru, aby zwiększyć wskaźnik sukcesu automatycznego wykrywania ryzykownych sytuacji."
    })
}
#[rocket::get("/it/rpa")]
fn rpa() -> Template {
    Template::render("it", context! {
        job_description: "Zespół RPA zajmuje się tworzeniem nowych rozwiązań automatyzacyjnych (botów) dla procesów, które obejmują całe SG i są wykonywane aktualnie przez ludzi. 
		Wykorzystujemy przy tym platformę Automation Anywhere oraz piszemy kod w Pythonie, VBS i JS. 
	 Nasza praca składa się na:
		Development nowych procesów, czyli najprościej pisanie nowego kodu.
		Maintanance stworzonych już botów, rozwiązywanie ticketów, naprawianie błędów, dodawanie nowych elementów.
		Monitoring statusu botów i platformy RPA."
    })
}
#[rocket::get("/md/mdc")]
fn mdc() -> Template {
    Template::render("md", context! {
        job_description: "Master data create"
    })
}
#[rocket::get("/md/mdg")]
fn mdg() -> Template {
    Template::render("md", context! {
        job_description: "Master data general"
    })
}
#[rocket::get("/md/mdm")]
fn mdm() -> Template {
    Template::render("md", context! {
        job_description: "Master data maintain"
    })
}
#[rocket::get("/ac/ap")]
fn ap() -> Template {
    Template::render("acp", context! {
        job_description: "Accounts Payable"
    })
}
#[rocket::get("/ac/ap/ndk")]
fn apndk() -> Template {
    Template::render("acp", context! {
        job_description: "AP NonDK"
    })
}
#[rocket::get("/ac/ap/dk")]
fn apdk() -> Template {
    Template::render("acp", context! {
        job_description: "AP DK"
    })
}
#[rocket::get("/ac/ap/c")]
fn apc() -> Template {
    Template::render("acp", context! {
        job_description: "AP Communication"
    })
}
#[rocket::get("/ac/p2p")]
fn p2p() -> Template {
    Template::render("ac", context! {
        job_description: "P2P"
    })
}
#[rocket::get("/ac/ar")]
fn ar() -> Template {
    Template::render("acr", context! {
        job_description: "Accounts Receivable"
    })
}

#[rocket::get("/ac/csde")]
fn csde() -> Template {
    Template::render("ac", context! {
        job_description: "Customer Service DE"
    })
}
#[rocket::get("/ac/gl/dk")]
fn gldk() -> Template {
    Template::render("acgl", context! {
        job_description: "GL DK"
    })
}
#[rocket::get("/ac/gl")]
fn gl() -> Template {
    Template::render("acgl", context! {
        job_description: "Genneral Ledger"
    })
}
#[rocket::get("/ac/gl/pl")]
fn glpl() -> Template {
    Template::render("acgl", context! {
        job_description: "GL PL"
    })
}
#[rocket::get("/ctrl/index")]
fn ctrl() -> Template {
    Template::render("ctrl", context! {
        job_description: "Controlling"
    })
}
#[rocket::get("/ctrl/fp")]
fn ctrlfp() -> Template {
    Template::render("ctrl", context! {
        job_description: "FP"
    })
}
#[rocket::get("/ctrl/ff")]
fn ctrlff() -> Template {
    Template::render("ctrl", context! {
        job_description: "FF"
    })
}
#[rocket::get("/hr/index")]
fn hr() -> Template {
    Template::render("hr", context! {
        job_description: "HR support,HR & Administration"
    })
}
#[rocket::get("/hr/hrs")]
fn hrs() -> Template {
    Template::render("hr", context! {
        job_description: "HR support"
    })
}
#[rocket::get("/hr/hra")]
fn hra() -> Template {
    Template::render("hr", context! {
        job_description: "HR & Administration"
    })
}
#[rocket::get("/payroll/index")]
fn payroll() -> Template {
    Template::render("payroll", context! {
        job_description: "Payroll"
    })
}
#[rocket::get("/payroll/pl")]
fn pl() -> Template {
    Template::render("payroll", context! {
        job_description: "Payroll"
    })
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![index,eco,desktop,sd,mon,cctv,bi,npl,change,application,rpa,payroll,hr,hrs,hra,ctrl,ctrlff,ctrlfp,gl,gldk,glpl,csde,ar,p2p,apndk,ap,apdk,apc,mdm,mdc,mdg,ac,md,it,pl])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
}
