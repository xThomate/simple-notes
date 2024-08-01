use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    loop {
        println!("Notizblock Menü:");
        println!("1. Notiz hinzufügen");
        println!("2. Notiz lesen");
        println!("3. Notiz löschen");
        println!("4. Notizen auflisten");
        println!("5. Beenden");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Fehler beim Lesen der Eingabe");
        let choice = choice.trim();

        match choice {
            "1" => add_note(),
            "2" => read_note(),
            "3" => delete_note(),
            "4" => list_notes(),
            "5" => break,
            _ => println!("Ungültige Auswahl, bitte versuche es erneut."),
        }
    }
}

fn add_note() {
    let mut note = String::new();
    println!("Gib den Notizinhalt ein:");
    io::stdin().read_line(&mut note).expect("Fehler beim Lesen der Notiz");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("notizen.txt")
        .expect("Fehler beim Öffnen der Datei");

    writeln!(file, "{}", note.trim()).expect("Fehler beim Schreiben in die Datei");
    println!("Notiz hinzugefügt!");
}

fn read_note() {
    let mut note = String::new();
    println!("Gib den Notizindex ein (beginnend bei 1):");
    io::stdin().read_line(&mut note).expect("Fehler beim Lesen der Eingabe");
    let index: usize = note.trim().parse().expect("Ungültiger Index");

    let contents = fs::read_to_string("notizen.txt").expect("Fehler beim Lesen der Datei");
    let notes: Vec<&str> = contents.lines().collect();

    if index > 0 && index <= notes.len() {
        println!("Notiz {}: {}", index, notes[index - 1]);
    } else {
        println!("Ungültiger Notizindex.");
    }
}

fn delete_note() {
    let mut note = String::new();
    println!("Gib den Notizindex ein (beginnend bei 1):");
    io::stdin().read_line(&mut note).expect("Fehler beim Lesen der Eingabe");
    let index: usize = note.trim().parse().expect("Ungültiger Index");

    let contents = fs::read_to_string("notizen.txt").expect("Fehler beim Lesen der Datei");
    let mut notes: Vec<&str> = contents.lines().collect();

    if index > 0 && index <= notes.len() {
        notes.remove(index - 1);
        fs::write("notizen.txt", notes.join("\n")).expect("Fehler beim Schreiben der Datei");
        println!("Notiz gelöscht!");
    } else {
        println!("Ungültiger Notizindex.");
    }
}

fn list_notes() {
    let contents = fs::read_to_string("notizen.txt").expect("Fehler beim Lesen der Datei");
    if contents.is_empty() {
        println!("Keine Notizen gefunden.");
    } else {
        println!("Notizen:");
        for (i, note) in contents.lines().enumerate() {
            println!("{}. {}", i + 1, note);
        }
    }
}
