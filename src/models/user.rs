// Serializar: Converter um struct Rust em um formato como JSON, YAML, TOML....
// Desserializar: Converter JSON (ou outro formato) em um struct Rust.
use serde::{Deserialize, Serialize};

// Forma elegante de tratar erros, na função eu jogo Result<>...
// Ele aceita qualquer erro que implemente std::error::Error, dessa forma usando Result, não preciso usar aquela sintaxe verbosa de Box...
use anyhow::Result;

// Você está dizendo: Implemente automaticamente os métodos para que essa struct seja convertida em JSON (Serialize) e de JSON para struct. (Deserialize)
// Sem isso você teria que implementar na mão uma parada GROTESCA DE GIGANTE para trabalhar com JSON e STRUCTS.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserPagination {
    pub users: Vec<User>,    // Lista de usuários retornados na página atual
    pub total_items: i64,    // Número total de usuários no banco de dados
    pub total_pages: i64,    // Número total de páginas
    pub current_page: i64,   // Página atual
    pub items_per_page: i64, // Quantidade de itens por página
}

impl User {
    // Não preciso usar <Option> porque mesemo se não tiver um usuário, a query me retornará um Vetor Vazio que já fala por si só.
    pub async fn get_all(pool: &deadpool_postgres::Pool) -> Result<Vec<Self>> {
        // ? --> Propagação de erro
        // .await --> Espera de forma assíncrona
        // Resumo: Espere de forma assíncrona uma conexão do pool. Se der certo, guarde na varíavel "client". Se der erro, propague!
        let client = pool.get().await?;

        let rows = client
            .query("SELECT id, nome, email FROM seguranca.usuarios", &[])
            .await?;

        // Rows --> é um <Vec<Row>>
        // .iter()     --> transforma esse vetor em algo que pode ser percorrido! (como um For)
        // .map()      --> prepara uma transformação para cada item quando o iterador for consumido (tou criando a estrutura final do meu OBJETO vamos dizer assim)
        // .colector() --> é isso que dispara a iteração de fato
        let users = rows
            .iter()
            .map(|row| Self {
                id: row.get("id"),
                name: row.get("nome"),
                email: row.get("email"),
            })
            .collect();

        Ok(users) // É basicamente um "return users"
    }

    // Option --> Tipo que representa a possibilidade de um valor existir ou não.
    // Ele pode retornar:
    // --> Some(T) --> Indica que a um valor presente
    // --> None --> Indica que não há nenhum valor

    // Result --> Tipo que representa o resultado de uma operação que pode falhar.
    // Ele tem dois estados:
    // --> Ok(T) --> Operação bem-sucedida e contém o valor T
    // --> Err(E) --> Operação falhou e contém um erro do tipo E

    //Combinado estou dizendo: "Essa operação pode falhar (Result),e, caso seja bem-sucedida (Ok), o valor retornado é um Option."
    // Em outras palavras, exatamente como está no controller:
    /*
        Ok(Some(value)): A operação foi bem-sucedida e retornou um valor.
        Ok(None): A operação foi bem-sucedida, mas não há valor disponível.
        Err(error): A operação falhou e retornou um erro.
    */
    pub async fn get_one(pool: &deadpool_postgres::Pool, id: i32) -> Result<Option<Self>> {
        let client = pool.get().await?;

        // query_opt --> Executa uma consulta SQL e retorna um <Result<Option>>
        let row = client
            .query_opt(
                "SELECT id, nome, email FROM seguranca.usuarios WHERE id = $1",
                &[&id],
            )
            .await?;

        let user = row.map(|row| Self {
            id: row.get("id"),
            name: row.get("nome"),
            email: row.get("email"),
        });

        Ok(user)
    }

    pub async fn get_all_paginated(
        pool: &deadpool_postgres::Pool,
        page: i64,
        items_per_page: i64,
    ) -> Result<UserPagination> {
        // Obter uma conexão do pool
        let client = pool.get().await?;

        // Calcular o OFFSET com base na página atual
        let offset = (page - 1) * items_per_page;

        // Consulta SQL para buscar os usuários da página atual
        let query = "
            SELECT id, name, email
            FROM seguranca.usuarios
            ORDER BY id
            LIMIT $1 OFFSET $2
        ";
        let rows = client.query(query, &[&items_per_page, &offset]).await?;

        // Mapear as linhas para a estrutura `User`
        let users: Vec<User> = rows
            .iter()
            .map(|row| User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
            })
            .collect();

        // Consulta SQL para contar o número total de usuários
        let total_items_query = "SELECT COUNT(*) FROM seguranca.usuarios";
        let total_items_row = client.query_one(total_items_query, &[]).await?;
        let total_items: i64 = total_items_row.get(0);

        // Calcular o número total de páginas
        let total_pages = (total_items + items_per_page - 1) / items_per_page;

        // Criar e retornar a estrutura `UserPagination`
        Ok(UserPagination {
            users,
            total_items,
            total_pages,
            current_page: page,
            items_per_page,
        })
    }
}
