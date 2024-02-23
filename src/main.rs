#[derive(Debug,Clone)]
pub enum Status {
    //感觉光用一个bool字段表示更好，但是这里可能考虑到未来图书的状态可能有更多吧:)
    Available(bool),
    Unavailable(bool),
}

#[derive(Debug)]
pub struct Book{
    //书名、作者、ISBN号、出版年份和状态（在库、借出）
    book_name: String,
    author: String,
    isbn: i32,
    publish_time: i32,
    status: Status,
}

#[derive(Debug)]
pub struct BookStore{
    bookstore: Vec<Book>,
}

impl BookStore{
    pub fn new() -> Self{
        let bookstore_new: Vec<Book> = Vec::new();
        BookStore{
            bookstore: bookstore_new
        }
    }

    pub fn add_book(&mut self, book_name: String, author: String, isbn: i32, publish_time: i32, status: Status){
        //添加图书：允许用户输入图书的详细信息并保存。
        self.bookstore.push(Book{
            book_name,
            author,
            isbn,
            publish_time,
            status
        })
    }

    pub fn delete_book(&mut self, isbn_cur: i32) {
        self.bookstore.retain(|book| book.isbn != isbn_cur);
    }

    pub fn update_book_status(&mut self, isbn_cur: i32, status_cur: Status){
        //借出与归还：更改图书的状态为借出或在库。
        if let Some(target_book) = self.bookstore.iter_mut().find(|book|{book.isbn == isbn_cur}){
            target_book.status = status_cur.clone();
        }
        else{
            println!("update_book_status error:book not found.")
        }
    }

    pub fn query_book<T: Fn(&Book) -> bool>(&self, query_info:T) -> Option<&Book> {
        //泛型查询功能：根据书名、作者或ISBN号查询图书。使用泛型函数支持不同类型的查询标准。
        self.bookstore.iter().find(|&book|query_info(book))
    }
    
    pub fn sort_bookstore<T: Fn(&mut Vec<Book>)>(&mut self, sort_strategy:T) {
        //自定义排序：允许用户根据自定义条件（如按出版年份、书名）对图书集合进行排序。使用闭包实现排序逻辑。
        //输入一个闭包作为参数，表示排序逻辑；执行sort
        sort_strategy(&mut self.bookstore);
    }
}

fn main() {
    println!("start to test library system...");

    let mut book_store = BookStore::new();

    //add book
    book_store.add_book(
        String::from("Imagined communities: reflections on the origin and spread of nationalism"),
        String::from("Benedict Richard O'Gorman Anderson"),
        135,
        2016,
        Status::Available(true),
    );

    book_store.add_book(
        String::from("Book1"),
        String::from("Tom"),
        155,
        2022,
        Status::Available(true),
    );

    book_store.add_book(
        String::from("Book2"),
        String::from("Sam"),
        145,
        2015,
        Status::Available(true),
    );

    println!("{:?}", book_store.bookstore[0].book_name);
    println!("{:?}", book_store.bookstore[1].book_name);
    println!("{:?}", book_store.bookstore[2].book_name);

    //update status
    book_store.update_book_status(135, Status::Unavailable(true));

    //delete book
    book_store.delete_book(135);

    println!("after delete book:{:?}", book_store.bookstore[0].book_name);

    //query book
    if let Some(target_book) = book_store.query_book(|book| book.isbn == 145){
        println!("query book:{:?}", target_book.book_name);
    }

    //sort bookstore
    book_store.sort_bookstore(|booklist: &mut Vec<Book>|{booklist.sort_by(|a, b| a.isbn.cmp(&b.isbn));
    });
    println!("after sort:{:?}", book_store.bookstore[0].book_name);
}

