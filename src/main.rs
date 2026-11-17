use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, Entry, Grid, Label, Orientation, Separator,
};

const APP_ID: &str = "org.portalit.calc3d";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Calculadora de Impressão 3D")
        .default_width(420)
        .default_height(520)
        .build();

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    let grid = Grid::builder()
        .row_spacing(10)
        .column_spacing(15)
        .build();

    let add_field = |row: i32, label_text: &str, default_val: &str| -> Entry {
        let label = Label::builder()
            .label(label_text)
            .halign(gtk4::Align::Start)
            .hexpand(true)
            .build();

        let entry = Entry::builder()
            .text(default_val)
            .halign(gtk4::Align::End)
            .width_chars(12)
            .build();

        grid.attach(&label, 0, row, 1, 1);
        grid.attach(&entry, 1, row, 1, 1);
        entry
    };

    let entry_preco_filamento = add_field(0, "Preço do filamento (R$/kg):", "58.00");
    let entry_gramas_lote = add_field(1, "Gramas de filamento (lote base):", "360");
    let entry_horas_lote = add_field(2, "Horas de impressão (lote base):", "10");
    let entry_pecas_lote = add_field(3, "Peças no lote base:", "8");
    let entry_qtd_pecas = add_field(4, "Quantidade de peças desejada:", "20");
    let entry_valor_hora = add_field(5, "Valor da hora de impressão (R$):", "5.00");
    let entry_watts = add_field(6, "Consumo da impressora (Watts):", "300");
    let entry_kwh = add_field(7, "Valor do kWh (R$):", "0.71");

    let btn_calcular = Button::builder()
        .label("Calcular Custo Total")
        .margin_top(10)
        .build();
    btn_calcular.add_css_class("suggested-action");

    let lbl_horas_totais = Label::builder()
        .label("Total de horas da impressora ligada: 25.00 h (900g de filamento)")
        .halign(gtk4::Align::Start)
        .build();

    let lbl_detalhes = Label::builder()
        .label("Filamento: R$ 52.20  |  Horas: R$ 125.00  |  Energia: R$ 5.33")
        .halign(gtk4::Align::Start)
        .build();

    let lbl_resultado = Label::builder()
        .label("<span size='x-large' weight='bold'>Total: R$ 182,525</span>")
        .use_markup(true)
        .margin_top(10)
        .build();

    // Monta a interface ANTES de mover os widgets para a closure
    main_box.append(&grid);
    main_box.append(&btn_calcular);
    main_box.append(&Separator::new(Orientation::Horizontal));
    main_box.append(&lbl_horas_totais);
    main_box.append(&lbl_detalhes);
    main_box.append(&lbl_resultado);

    // Agora o move |_| pode assumir a posse dos labels sem erro
    btn_calcular.connect_clicked(move |_| {
        let parse_val = |entry: &Entry| -> f64 {
            entry
                .text()
                .replace(',', ".")
                .trim()
                .parse::<f64>()
                .unwrap_or(0.0)
        };

        let preco_kg = parse_val(&entry_preco_filamento);
        let gramas_lote = parse_val(&entry_gramas_lote);
        let horas_lote = parse_val(&entry_horas_lote);
        let pecas_lote = parse_val(&entry_pecas_lote);
        let qtd_pecas = parse_val(&entry_qtd_pecas);
        let valor_hora = parse_val(&entry_valor_hora);
        let watts = parse_val(&entry_watts);
        let valor_kwh = parse_val(&entry_kwh);

        if pecas_lote <= 0.0 {
            lbl_resultado.set_markup("<span color='red'>Peças no lote deve ser > 0</span>");
            return;
        }

        let fator_producao = qtd_pecas / pecas_lote;
        let gramas_totais = gramas_lote * fator_producao;
        let horas_totais_ligada = horas_lote * fator_producao;

        let custo_filamento = (preco_kg / 1000.0) * gramas_totais;
        let custo_maquina = valor_hora * horas_totais_ligada;
        let custo_energia = ((watts * horas_totais_ligada) / 1000.0) * valor_kwh;

        let total = custo_filamento + custo_maquina + custo_energia;

        lbl_horas_totais.set_text(&format!(
            "Total de horas da impressora ligada: {:.2} h ({:.0}g de filamento)",
            horas_totais_ligada, gramas_totais
        ));

        lbl_detalhes.set_text(&format!(
            "Filamento: R$ {:.2}  |  Horas: R$ {:.2}  |  Energia: R$ {:.2}",
            custo_filamento, custo_maquina, custo_energia
        ));

        lbl_resultado.set_markup(&format!(
            "<span size='x-large' weight='bold'>Total: R$ {:.3}</span>",
            total
        ));
    });

    window.set_child(Some(&main_box));
    window.present();
}