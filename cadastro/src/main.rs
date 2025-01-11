/*
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/

use std::{collections::HashMap, io};

fn main() {
    let mut cadastro: HashMap<String, Vec<String>>= HashMap::new();

    let mut input_main: String = String::new();
    let mut input_dept: String = String::new();

    loop {
        println!("\n-=-=-=-=-=-= Text interface register =-=-=-=-=-=-\n");
        println!("[1] - Manipular cadastro");
        println!("[2] - Listar toda empresa");
        println!("[3] - Listar departamento");
        println!("[Q] - Sair");

        input_main.clear();
        io::stdin()
        .read_line(&mut input_main)
        .expect("Erro ao ler entrada\n");

        match input_main.trim().to_uppercase().as_str() {
            "1" => {
                let resultado = receber_operacao();
                let acao = &resultado[0];
                let funcionario = &resultado[1];
                let departamento = &resultado[2];
                let res = manipular_cadastro(&mut cadastro, acao, funcionario, departamento);
            
                if !res {
                    println!("Falha na operacao, tente novamente");
                    continue;
                }
            },
            "2" => {
                for (dept, lista) in &cadastro {
                    let nome_dept = dept.to_uppercase();
                    println!("{nome_dept}");
                    print!("\t");
                    for func in lista {
                        print!("{func} ");
                    }
                    println!("\n");
                }
            },
            "3" => {
                loop{
                    println!("\nDepartamentos existentes:");
                    for dept in cadastro.keys() {
                        print!("{dept} ");
                    }
                    println!();

                    input_dept.clear();
                    io::stdin()
                    .read_line(&mut input_dept)
                    .expect("Erro ao ler departamento");

                    let nome_dept = input_dept.trim().to_lowercase();

                    if let Some(dept_list) = cadastro.get(nome_dept.as_str()) {
                        let nome = nome_dept.to_uppercase();
                        println!("{nome}");
                        print!("\t");
                        for func in dept_list {
                            print!("{func} ");
                        }
                        println!("\n");
                        break;
                    } else {
                        println!("\nInsira um nome válido de departamento!");
                    }
                }
            },
            "Q" => break,
            _ => println!("Insira uma opção válida"),
        }
    }
}

fn receber_operacao () -> Vec<String> {
    let mut entrada = String::new();
    let mut acao: String;
    let mut funcionario: String;
    let mut departamento: String;

    loop {
        println!("\nInsira sua operação");

        entrada.clear();
        io::stdin()
        .read_line(&mut entrada).
        expect("Erro ao ler entrada\n");
        
        let mut operacao = entrada.split_ascii_whitespace();

        match operacao.next() {
            Some(a) => acao = a.to_lowercase(),
            _ => {
                println!("Insira uma operação válida!");
                continue;
            }
        }

        match operacao.next() {
            Some(f) => funcionario = f.to_lowercase(),
            _ => {
                println!("Insira um nome de funcionário!");
                continue;
            } 
        }

        //pulanco o to ou from
        operacao.next();

        match operacao.next() {
            Some(d) => departamento = d.to_lowercase(),
            _ => {
                println!("Insira o nome do departamento!");
                continue;
            }
        }

        match acao.as_str() {
            "add" | "remove" => (),
            _ => {
                println!("[Erro] Insira uma operação válida!");
                continue;
            }
        }
        break;
    }
    
    let res = vec![acao, funcionario, departamento];
    return res
}

fn manipular_cadastro(cadastro: &mut HashMap<String, Vec<String>>, acao: &String, funcionario: &String, departamento: &String) -> bool {

    match acao.as_str() {
        "add" => {
            let dept_list = cadastro.entry(departamento.clone()).or_insert_with(Vec::new);
            if dept_list.contains(&funcionario) {
                println!("Funcionario ja esta cadastrado em {departamento}");
                return true;
            }

            dept_list.push(funcionario.clone());
            println!("Funcionario adicionado com sucesso!");
            return true;
        },
        "remove" => {
            if let Some(dept_list) = cadastro.get_mut(departamento.as_str()) {
                if dept_list.contains(&funcionario) {
                    dept_list.retain(|f| *f != funcionario.clone());
                    println!("Funcionario removido com sucesso!");
                    return true;
                }
                println!("Funcionario não está cadastrado em {departamento}");
                return true;
            }
        }
        _ => {
            println!("[Erro] erro ao detectar operação");
            return false
        }
    }
    return true
}