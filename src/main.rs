use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, Entry, Grid, Label, Orientation, Separator,
};

const APP_ID: &str = "org.exemplo.Calculadora3D";

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
        .default_height(480)
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

    // Campos diretos por peça
    let entry_gramas_peca = add_field(0, "Gramas por peça (g):", "77.90");
    let entry_horas_peca = add_field(1, "Horas por peça (h):", "4.4");
    let entry_qtd_pecas = add_field(2, "Quantidade de peças:", "1");
    let entry_preco_filamento = add_field(3, "Preço do filamento (R$/kg):", "80.00");
    let entry_preco_hora = add_field(4, "Preço por hora de impressão (R$):", "5.00");
    let entry_watts = add_field(5, "Watts da impressora (W):", "300");
    let entry_kwh = add_field(6, "Preço do kWh (R$):", "0.71");

    let btn_calcular = Button::builder()
        .label("Calcular Custo")
        .margin_top(10)
        .build();
    btn_calcular.add_css_class("suggested-action");

    let lbl_totais_fisicos = Label::builder()
        .label("Total ligada: 4.40 h | Filamento total: 77.9g")
        .halign(gtk4::Align::Start)
        .build();

    let lbl_detalhes = Label::builder()
        .label("Filamento: R$ 6.23 | Horas: R$ 22.00 | Energia: R$ 0.94")
        .halign(gtk4::Align::Start)
        .build();

    let lbl_resultado = Label::builder()
        .label("<span size='x-large' weight='bold'>Total: R$ 29.17 (R$ 29.17 / peça)</span>")
        .use_markup(true)
        .margin_top(10)
        .build();

    main_box.append(&grid);
    main_box.append(&btn_calcular);
    main_box.append(&Separator::new(Orientation::Horizontal));
    main_box.append(&lbl_totais_fisicos);
    main_box.append(&lbl_detalhes);
    main_box.append(&lbl_resultado);

    btn_calcular.connect_clicked(move |_| {
        let parse_val = |entry: &Entry| -> f64 {
            entry
                .text()
                .replace(',', ".")
                .trim()
                .parse::<f64>()
                .unwrap_or(0.0)
        };

        let gramas_peca = parse_val(&entry_gramas_peca);
        let horas_peca = parse_val(&entry_horas_peca);
        let qtd_pecas = parse_val(&entry_qtd_pecas);
        let preco_kg = parse_val(&entry_preco_filamento);
        let preco_hora = parse_val(&entry_preco_hora);
        let watts = parse_val(&entry_watts);
        let preco_kwh = parse_val(&entry_kwh);

        if qtd_pecas <= 0.0 {
            lbl_resultado.set_markup("<span color='red'>A quantidade deve ser maior que 0</span>");
            return;
        }

        // Totais diretos
        let gramas_totais = gramas_peca * qtd_pecas;
        let horas_totais = horas_peca * qtd_pecas;

        // Custos
        let custo_filamento = (preco_kg / 1000.0) * gramas_totais;
        let custo_horas = preco_hora * horas_totais;
        let custo_energia = ((watts * horas_totais) / 1000.0) * preco_kwh;

        let total = custo_filamento + custo_horas + custo_energia;
        let valor_unitario = total / qtd_pecas;

        lbl_totais_fisicos.set_text(&format!(
            "Total ligada: {:.2} h | Filamento total: {:.1}g",
            horas_totais, gramas_totais
        ));

        lbl_detalhes.set_text(&format!(
            "Filamento: R$ {:.2} | Horas: R$ {:.2} | Energia: R$ {:.2}",
            custo_filamento, custo_horas, custo_energia
        ));

        lbl_resultado.set_markup(&format!(
            "<span size='x-large' weight='bold'>Total: R$ {:.2} (R$ {:.2} / un)</span>",
            total, valor_unitario
        ));
    });

    window.set_child(Some(&main_box));
    window.present();
}