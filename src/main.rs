use chrono::Local;
use std::env;
use std::fs;
use std::path::Path;

const LOGIN: &str = "USER";
const EMAIL: &str = "E-MAIL@42.fr";

const ASCII_ART: [&str; 7] = [
    "        :::      ::::::::",
    "      :+:      :+:    :+:",
    "    +:+ +:+         +:+  ",
    "  +#+  +:+       +#+     ",
    "+#+#+#+#+#+   +#+        ",
    "     #+#    #+#          ",
    "    ###    ########.fr   ",
];

fn get_comment_chars(filename: &str) -> (&'static str, &'static str, char) {
    let ext = Path::new(filename).extension().and_then(|e| e.to_str()).unwrap_or("");
    match ext {
        "c" | "cpp" | "h" | "hpp" | "js" | "php" | "go" | "rs" | "java" => ("/*", "*/", '*'),
        "html" | "xml" => ("<!--", "-->", '*'),
        "mk" | "py" | "pl" | "rb" | "sh" | "yml" | "yaml" | "conf" => ("#", "#", '*'),
        "tex" => ("%", "%", '*'),
        "el" | "lisp" | "s" => (";", ";", '*'),
        "lua" | "sql" => ("--", "--", '-'),
        _ => ("#", "#", '*'),
    }
}

fn format_line(left: &str, right: &str, start: &str, end: &str) -> String {
    let margin = 5;
    let total_len = 80;
    let content_width = total_len - (margin * 2);
    
    let left_trunc = if left.len() + right.len() > content_width {
        &left[..content_width - right.len()]
    } else {
        left
    };

    let pad_start = " ".repeat(margin - start.len());
    let spacer = " ".repeat(content_width - left_trunc.len() - right.len());
    let pad_end = " ".repeat(margin - end.len());

    format!("{}{}{}{}{}{}{}", start, pad_start, left_trunc, spacer, right, pad_end, end)
}

fn generate_header(filename: &str, existing_created: Option<String>) -> Vec<String> {
    let (start, end, fill_char) = get_comment_chars(filename);
    let date_str = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let fill_len = 80 - start.len() - end.len() - 2;
    let solid_line = format!("{} {} {}", start, fill_char.to_string().repeat(fill_len), end);
    let empty_line = format_line("", "", start, end);

    let created_str = if let Some(c) = existing_created {
        c
    } else {
        format!("Created: {} by {}", date_str, LOGIN)
    };

    vec![
        solid_line.clone(),
        empty_line.clone(),
        format_line("", ASCII_ART[0], start, end),
        format_line(filename, ASCII_ART[1], start, end),
        format_line("", ASCII_ART[2], start, end),
        format_line(&format!("By: {} <{}>", LOGIN, EMAIL), ASCII_ART[3], start, end),
        format_line("", ASCII_ART[4], start, end),
        format_line(&created_str, ASCII_ART[5], start, end),
        format_line(&format!("Updated: {} by {}", date_str, LOGIN), ASCII_ART[6], start, end),
        empty_line.clone(),
        solid_line,
    ]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: 42header <file_path>");
        return;
    }

    let filepath = &args[1];
    let filename = Path::new(filepath).file_name().unwrap().to_str().unwrap();
    let content = fs::read_to_string(filepath).unwrap_or_else(|_| String::new());
    
    let lines: Vec<&str> = content.lines().collect();

    let (start, _, _) = get_comment_chars(filename);
    let update_prefix = format!("{}{}", start, " ".repeat(5 - start.len()));

    let mut is_update = false;
    let mut existing_created = None;

    if lines.len() >= 11 && lines[8].contains("Updated:") && lines[8].starts_with(&update_prefix) {
        is_update = true;
        // Extrai a data de criação existente na linha 8 (índice 7)
        let created_line = lines[7].trim();
        if let Some(idx) = created_line.find("Created:") {
            let parts: Vec<&str> = created_line[idx..].split("   ").collect();
            existing_created = Some(parts[0].to_string());
        }
    }

    // CORREÇÃO 1.1: Atualizada a chamada da função para refletir a nova assinatura.
    let header = generate_header(filename, existing_created);

    if is_update {
        // Substitui as primeiras 11 linhas
        let mut new_content = header.join("\n");
        new_content.push('\n');
        new_content.push_str(&lines[11..].join("\n"));
        fs::write(filepath, new_content).expect("Failed to write file");
    } else {
        // Insere no topo
        let mut new_content = header.join("\n");
        new_content.push_str("\n\n");
        new_content.push_str(&content);
        fs::write(filepath, new_content).expect("Failed to write file");
    }
}
