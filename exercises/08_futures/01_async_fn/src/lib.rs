use tokio::net::TcpListener;

// TODO: Напишите echo-сервер, который принимает входящие TCP-соединения
//  и отправляет полученные данные обратно клиенту.
//  Завершив обработку соединения, `echo` не должен возвращаться, а должен
//  продолжать принимать новые соединения.
//
// Подсказка: для реализации echo-сервера используйте структуры и методы `tokio`.
// В частности:
// - `tokio::net::TcpListener::accept` для обработки следующего входящего соединения
// - `tokio::net::TcpStream::split` для получения reader и writer из сокета
// - `tokio::io::copy` для копирования данных из reader в writer
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut reader, mut writer) = socket.split();

            // Отправляем запрос
            writer.write_all(request.as_bytes()).await.unwrap();
            // Закрываем сторону записи сокета
            writer.shutdown().await.unwrap();

            // Читаем ответ
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}
