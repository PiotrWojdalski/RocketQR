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
        job_description: "Zespół zarządza i konfiguruje systemy operacyjne firmy oparte na Linuxie (RedHat, CentOS) oraz zarządza konfiguracjami systemu Vikinging. Platform & Cloud Operations wykonuje także testy obciążeniowe infrastruktury e-commerce i na ich podstawie tworzy scenariusze w JavaScript, zbiera wyniki i prezentuje je w Grafanie. Pracownicy są także odpowiedzialni za tworzenie dokumentacji do infrastruktury, systemów i procedur, którymi zarządzają."
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
        job_description: "Zespół zajmuje się wspieraniem biznesu w obszarze danych aplikacji Business Intelligence. Wsparcie polega na odpowiadaniu na wszelkiego rodzaju zapytania, rozwiązywaniu problemów użytkowników oraz czynnym uczestnictwie w projektach wdrażania nowych funkcjonalności systemu. BI Support bada potrzeby biznesu, a następnie przekłada je na język IT i w ten sposób buduje oraz testuje nowe rozwiązania i funkcje."
    })
}
#[rocket::get("/it/desktop")]
fn desktop() -> Template {
    Template::render("it", context! {
        job_description: "Zespół utrzymuje i obsługuje stacje robocze, w tym systemy operacyjne, sterowniki i pakiety oprogramowania wykorzystywane w organizacji. Zadaniem pracowników jest także wsparcie i obsługa systemu zarządzania ThinClient i PDA, testowanie i udostępnianie poprawek bezpieczeństwa systemu operacyjnego oraz zapewnienie wsparcia drugiego poziomu dla obsługiwanego oprogramowania komputerowego. Desktop Managment dba także o prowadzenie aktualnej dokumentacji w głównych obszarach swoich kompetencji. Zespół współpracuje z IT Sourcing Partnerem."
    })
}
#[rocket::get("/it/sd")]
fn sd() -> Template {
    Template::render("it", context! {
        job_description: "Zespół zapewnia pierwszą linię wsparcia i na co dzień kontaktuje się bezpośrednio z pracownikami różnych formatów i jednostek skupionych w Salling Group. Główne zadania to rozwiązywanie bieżących problemów technicznych związanych z działaniem rozległej sieci systemów informatycznych (SAP, CISCO, Windows, Linux). Dodatkowo zespół odpowiada za ewidencjonowanie błędów systemowych w oparciu o ramy czasowe, wspieranie duńskich zespołów drugiej linii wsparcia, pośredni i bezpośredni kontakt z użytkownikiem oraz tworzenie i edycję dokumentacji technicznej."
    })
}
#[rocket::get("/it/mon")]
fn mon() -> Template {
    Template::render("it", context! {
        job_description: "Głównym zadaniem zespołu jest monitorowanie działania takich systemów jak: statusy zamówień, danych sprzedaży do PE0, realizacji F&R, monitorowanie ESL, WiFi i sieci, sprzedaży godzinowej oraz reagowanie na alerty generowane przez te systemy. Najważniejszym obowiązkiem członków zespołu jest zapobieganie błędom, zanim dotkną one użytkownika. Zespół pracuje w cyklu 24h/365."
    })
}
#[rocket::get("/it/npl")]
fn npl() -> Template {
    Template::render("it", context! {
        job_description: "NPL to pierwsza linia wsparcia dla sieci Netto Polska. Zespół wspiera sklepy, magazyny i HQ poprzez rozwiązywanie problemów technicznych i operacyjnych. Netto PL posiada drugą linię wsparcia w Warszawie oraz drugą i trzecią linię wsparcia w Motańcu."
    })
}
#[rocket::get("/it/change")]
fn change() -> Template {
    Template::render("it", context! {
        job_description: "Jednoosobowy zespół odpowiadający za koordynację zmian zachodzących w środowisku IT,   np. kategoryzowanie zmian i przygotowywanie ich w sposób, który zminimalizuje ryzyko powstania problemów dla biznesu (sklepów, magazynów, biura)."
    })
}
#[rocket::get("/it/cctv")]
fn cctv() -> Template {
    Template::render("it", context! {
        job_description: "Zespół obsługuje i monitoruje system CCTV w celu oceny sytuacji wysokiego ryzyka. W wypadku wykrycia oszustwa zgłasza to oraz bierze udział w określaniu i rozwiązywaniu problemów na podstawie istniejących procedur. Kolejnym zadaniem CCTV Monitoring jest kalibracja systemu nadzoru, co pozwala zwiększyć wskaźnik sukcesu automatycznego wykrywania ryzykownych sytuacji."
    })
}
#[rocket::get("/it/rpa")]
fn rpa() -> Template {
    Template::render("it", context! {
        job_description: "Zespół zajmuje się tworzeniem nowych rozwiązań automatyzacyjnych (botów) dla procesów, które obejmują całą spółkę Salling Group i są wykonywane aktualnie przez ludzi. Do ich

        tworzenia zespół wykorzystuje platformę Automation Anywhere. Automation na bieżąco zarządza stworzonymi już botami, rozwiązuje tickety i naprawia błędy oraz monitoruje status botów i platformy."
    })
}
#[rocket::get("/md/mdc")]
fn mdc() -> Template {
    Template::render("md", context! {
        job_description: "Zespół rejestruje artykuły na podstawie kart od kupca. Wprowadza dane logistyczne związane z produktem (np. ile sztuk jest w kartonie, jakie są wymiary – np. by sklep przygotował na nie miejsce w magazynie). W niektórych obszarach tworzy zamówienia asortymentu do sklepów na podstawie tzw. Purchase Order. Zespół MD Create zajmuje się również rejestracją artykułów sportowych oraz asortymentu stałego dla Netto PL, Netto DE oraz duńskich odpowiedników tych sklepów. Do zadań zespołu Create należy także dyżurowanie dla sklepu Netto. Dyżury odbywają się w tygodniu w godzinach 6-8 oraz w soboty, dzięki czemu zespół pomaga jak najszybciej rozwiązywać problemy pojawiające się na sklepach.",
        job_1: "W zespole MD Create znajduje się także zespół Netto Pomoc, który zajmuje obsługą zgłoszeń sklepowych, która polega na dbaniu o prawidłowy przepływ komunikacji pomiędzy Działami. Obsługa zgłoszeń odbywa się w systemie Service Now. Zespół odpowiada także, za obsługę aplikacji MyNetto i najważniejsze informacje, które powinny docierać do sklepów od Administracji. Netto Pomoc odpowiada za przygotowanie roboczej wersji kalendarza operacyjnego związanego z działaniami operacyjnymi w sklepach. Po otrzymaniu ostatecznej wersji kalendarza działania są publikowane w aplikacji."
    })
}
#[rocket::get("/md/mdg")]
fn mdg() -> Template {
    Template::render("md", context! {
        job_description: "Zespół jest odpowiedzialny m.in. za proces tworzenia i aktualizacji danych dla dostawców oraz za tworzenie i zmiany na artykułach ZADM i Intersourcing. Zespół odpowiada także za Check i raporty dla całego działu MD, czyli weryfikuje poprawność danych oraz tworzy wyrywkowe i cykliczne raporty zarówno dla całej Master Daty, jak i organizacji np. MAP check."
    })
}
#[rocket::get("/md/mdm")]
fn mdm() -> Template {
    Template::render("md", context! {
        job_description: "MD Maintain wprowadza w systemie SAP zmiany na obecnych produktach np. dane logistyczne, kody, ceny, promocje, szyldy cenowe. Zmiany wprowadzane są na obszarach duńskich, Netto PL oraz Netto DE. Zespół jest również odpowiedzialny za trouble shooting – szukanie rozwiązań i przyjmowanie zapytań dla różnego rodzaju spraw, które się pojawiają. (np. jeśli jakiś produkt nie działa na kasie). Do zadań MDM należy także obsługa skrzynki Supplier Data Colaboration (SDC)

        i zajmowanie się sprawami związanymi z ustawieniami Consignment na artykułach. "
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
        job_description: "Zespół odpowiada za księgowanie dokumentów z obszaru Niemiec i Polski. Zadaniem zespołu jest weryfikacja oraz rejestracja polskich i niemieckich faktur w skanerze dokumentów Smart Scan. Zespół ustala, czy dane dostawcy są prawidłowe, czy wpisane jest właściwe konto bankowe oraz czy fakturę wystawiono na odpowiednie zamówienie, które w dalszym procesie zostanie zaksięgowane i opłacone. Księgowanie w zespole jest podzielone na faktury kosztowe (Coupa) i towarowe (SAP)."
    })
}
#[rocket::get("/ac/ap/dk")]
fn apdk() -> Template {
    Template::render("acp", context! {
        job_description: "Zespół odpowiada za księgowanie dokumentów z obszaru duńskiego. Zadaniem zespołu jest weryfikacja oraz rejestracja duńskich faktur w aplikacji Smart Scan. Zespół ustala, czy dane dostawcy są prawidłowe, czy wpisane jest właściwe konto bankowe oraz, czy fakturę wystawiono na odpowiednie zamówienie, które w dalszym procesie zostanie zaksięgowane i opłacone. Księgowanie w zespole jest podzielone na faktury kosztowe (Coupa) i towarowe (SAP). Zespół księguje także dokumenty transportowe (Freight) oraz weryfikuje, księguje i rozlicza konta PBS."
    })
}
#[rocket::get("/ac/ap/c")]
fn apc() -> Template {
    Template::render("acp", context! {
        job_description: "Zespół zajmuje się komunikacją z dostawcami oraz biznesem. Helpdesk wyjaśnia ewentualne różnice oraz kontaktuje się z innymi departamentami kupców, sklepów oraz magazynów. Zespół odpowiada także za egzekwowanie zaległych należności od dostawców. W zespole komunikacji znajduje się podzespół Payments, którego zadaniem jest realizowanie przelewów wychodzących dla całej spółki Salling Group."
    })
}
#[rocket::get("/ac/p2p")]
fn p2p() -> Template {
    Template::render("ac", context! {
        job_description: "Zespół, który zarządza zakupami oraz kosztami firmy w systemie Coupa. Od zakupów do płatności – zespół odpowiedzialny jest na wdrożenie zawartości i zarządzanie systemem. Na zarządzanej platformie prowadzone są aukcje gdzie firmy wygrywają świadczenie usług dla spółki (zostają głównym dostawca np. produktów)."
    })
}
#[rocket::get("/ac/ar")]
fn ar() -> Template {
    Template::render("acr", context! {
        job_description: "Zadaniem zespołu jest weryfikaJcja czy suma otrzymanych środków pieniężnych jest zgodna z sumą należną figurującą w naszym systemie księgowym. Niezależnie od tego, czy jest to obrót gotówkowy w sklepie, płatność kartą czy bonami. Zespół AR na bieżąco monitoruje dane ze sklepów i innych działów oraz śledzi stan na rachunkach bankowych. Po weryfikacji sald zespół raportuje je do Działu Należności, który z kolei je przelicza i sprawdza, czy suma pieniędzy za zakupy klientów zgadza się ze stanem konta firmowego. W wypadku nieprawidłowości zespół wyjaśnia je z Działem Audytu. Zespół współpracuje ze wszystkimi formatami Salling Group w prowadzeniu wymienionych działań."
    })
}

#[rocket::get("/mdc/csde")]
fn csde() -> Template {
    Template::render("ac", context! {
        job_description: "Jest to zespół, który wspiera niemiecki obszar sklepów Netto w kontakcie z klientem zewnętrznym. Obowiązki zespołu głównie dotyczą obsługi systemu ZenDesk, gdzie trafiają różnego rodzaju zapytania od klientów sklepów Netto na które trzeba odpowiedzieć np. Anulowanie mandatu za parking, Reklamacje produktów – uszkodzonych, niezgodnych z opisem, nieświeżych oraz zapytania o produkty w kolejnych gazetkach."
    })
}
#[rocket::get("/ac/gl/dk")]
fn gldk() -> Template {
    Template::render("acgl", context! {
        job_description: "Zespół jest odpowiedzialny za procesy związane z zamknięciem miesiąca, kwartału oraz roku np. konsolidacje, dystrybucje kosztów, reinvoicing, księgowanie amortyzacji i leasingów wewnętrznych dla całej spółki Salling Group. GL DK przygotowuje również dokumentacje do electricity duty, excise duty i import duty. Zespół weryfikuje także techniczne księgowania intercompany, porządkuje frachty, współpracuje z zespołem automation przy obsłudze GLSU. Do zadań GL DK należy również rozliczanie środków trwałych. Zespół odpowiada też za część kont bankowych, wprowadza zmiany w NAKISIE oraz robi księgowania związane z IFRS 16. Współpracuje zarówno z Danią, Polską jak i Niemcami."
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
        job_description: "Zespół odpowiada za procesy związane z zamknięciem miesiąca, kwartału oraz roku np. konsolidacje, dystrybucje kosztów, reinvoicing, księgowanie amortyzacji i leasingów wewnętrznych w obszarze polskim. Zespół po dokonaniu weryfikacji księgowań, rozliczeń podatku od towarów i usług oraz po kalkulacji podatku dochodowego od osób prawnych, uzgadnia konta i raportuje do Narodowego Banku Polskiego i Głównego Urzędu Statystycznego. GL PL rozlicza także środki trwałe w obszarze polskim, fakturuje dzierżawców wynajmujących powierzchnie w sklepach Netto, dokonuje rezerw na koniec miesiąca oraz sporządza roczne sprawozdanie finansowe."
    })
}
#[rocket::get("/ctrl/index")]
fn ctrl() -> Template {
    Template::render("ctrl", context! {
        job_description: "Zadaniem działu jest raportowanie, przygotowywanie budżetów, tworzenie prognoz krótko i długoterminowych oraz przygotowywanie szczegółowych analiz na potrzeby wewnętrzne. Controlling jest podzielony na dwa obszary:",
        job_1: "FA/SA/CA/LOGISTICS: - Zespół zajmuje się analizowaniem danych dla kluczowych osób decyzyjnych w Salling Group, Działu IT, Działu Projektów Wewnętrznych, Działu Inwestycji oraz dla Działu Zakupów i wszystkich formatów Salling Group.",
        job_2: "FP&A/CF/NF/IT/CAPEX: - Zespół zajmuje się analizowaniem danych z obszaru logistyki, audytu wewnętrznego oraz danymi z Działu Zakupów i wszystkich formatów Salling Group."
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
        job_description: "Zespół zajmuje się wsparciem administracyjnych działań związanych z duńskim obszarem

        HR poprzez wprowadzanie i dokonywanie zmian danych osobowych w systemie Emploee
        
        Central oraz SAP HR. Do głównych obszarów pracy należą: administracja testów dla
        
        pracowników i kandydatów, tworzenie i przetwarzanie kontraktów dla duńskich
        
        pracowników oraz rozliczanie zwrotów z urlopów macierzyńskich, zwolnień chorobowych
        
        dla pracowników Salling Group w Danii."
    })
}
#[rocket::get("/hr/hra")]
fn hra() -> Template {
    Template::render("hr", context! {
        job_description: "Zespół odpowiedzialny jest za kompleksowe wsparcie SSC w szeroko rozumianym obszarze HR. Zadania obejmują między innymi: prowadzenie procesów rekrutacyjnych, organizację szkoleń, prowadzenie dokumentacji pracowników SSC oraz projekty HR. Dział odpowiada również za działania z zakresu Employer Brandingu oraz koordynuje wewnętrzne benefity."
    })
}
#[rocket::get("/payroll/index")]
fn payroll() -> Template {
    Template::render("payroll", context! {
        job_description: "Payroll zajmuje się obsługą kadrowo-płacową pracowników sklepów Netto. Odpowiada za

        zatrudnianie pracowników, przygotowanie umów o pracę oraz poprawne naliczanie wynagrodzeń pracowników. Dział odpowiada przed instytucjami państwowymi takimi jak ZUS czy Urząd Skarbowy za dostarczanie poprawnych danych dotyczących wysokości naliczonego podatku lub składek emerytalno-rentowych. Stoi na straży przestrzegania prawa pracy, dba o kompletność akt personalnych oraz przygotowanie raportu płacowych dla biznesu. Zespół Payroll jest podzielony na dwa obszary:",
        job_1: "Netto Indygo
        TL: Katarzyna Wabiszewicz 
        Zespół odpowiada za obsługę kadrowo – płacową byłych pracowników Tesco.",
        job_2: "Netto Yellow 
        TL: Anna Kowalewska.
        Obsługa kadrowo – płacowa tego zespołu obejmuje pracowników Netto  "

    })
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![index,eco,desktop,sd,mon,cctv,bi,npl,change,application,rpa,payroll,hr,hrs,hra,ctrl,gl,gldk,glpl,csde,ar,p2p,apndk,ap,apdk,apc,mdm,mdc,mdg,ac,md,it])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
}
