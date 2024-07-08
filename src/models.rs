#[allow(dead_code)]
struct Customer {
    uid: String,
    name: String,
    phone: String,
    email: String,
    address: Option<Address>,
    order_history: Vec<String>,
}

#[allow(dead_code)]
struct Address {
    street: String,
    street_2: Option<String>,
    city: String,
    prefecture: String,
    postal_code: String,
}

#[allow(dead_code)]
struct Product {
    uid: String,
    name: String,
    images: Vec<Image>,
    description: String,
    r#type: ProductType,
    recipe_link: String,
}

#[allow(dead_code)]
struct Image(String);

#[allow(dead_code)]
enum ProductType {
    Zephyr { quantity: u8, price: u16 },
    Cake { diameter: u8, price: u16 },
    CupCake { quantity: u8, price: u16 },
}

#[allow(dead_code)]
struct Order {
    uid: String,
    customer_id: String,
    products: Vec<Product>,
    delivery_date: String,
    receipt_method: ReceiptMethod,
    comments: Vec<String>,
    total: u16,
    status: OrderStatus,
}

#[allow(dead_code)]
enum ReceiptMethod {
    PickUp,
    Delivery { price: u16 },
    Mail { is_shipped: bool, price: u16 },
}

#[allow(dead_code)]
enum OrderStatus {
    Requested,
    Approved,
    Paid,
    Received,
    Closed,
    Cancelled { refund_status: RefundStatus },
}

#[allow(dead_code)]
enum RefundStatus {
    Requested { refunded: bool },
    None,
}
