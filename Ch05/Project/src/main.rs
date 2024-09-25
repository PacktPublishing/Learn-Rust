// Define a PaymentMethod trait that has two methods: process_payment and validate_details
trait PaymentMethod {
    fn process_payment(&self, amount: f64) -> Result<(), String>;
    fn validate_details(&self) -> bool;
}

// Struct for CreditCard payment method
struct CreditCard {
    number: String,
    expiry_date: String,
    cvv: String,
}

// Implement the PaymentMethod trait for CreditCard
impl PaymentMethod for CreditCard {
    fn process_payment(&self, amount: f64) -> Result<(), String> {
        if !self.validate_details() {
            return Err("Credit card details are invalid.".to_string());
        }
        if amount < 0.0 {
            return Err("Invalid amount, cannot process negative payment.".to_string());
        }
        println!("Processing credit card payment of ${:.2}", amount);
        Ok(())
    }

    fn validate_details(&self) -> bool {
        // Here we check if the length of the credit card number is 16 etc.
        self.number.len() == 16 && self.expiry_date.len() == 5 && self.cvv.len() == 3
    }
}

// Struct for Paypal payment method
struct Paypal {
    email: String,
    password: String,
}

// Implement the PaymentMethod trait for Paypal
impl PaymentMethod for Paypal {
    fn process_payment(&self, amount: f64) -> Result<(), String> {
        if !self.validate_details() {
            return Err("Paypal account details are invalid.".to_string());
        }
        if amount < 0.0 {
            return Err("Invalid amount, cannot process negative payment.".to_string());
        }
        println!("Processing Paypal payment of ${:.2}", amount);
        Ok(())
    }

    fn validate_details(&self) -> bool {
        // Simple email validation
        (self.email.contains('@') && self.email.contains('.') && self.email.len() >= 5)
            // Validate Bitcoin password is filled
            && (self.password.len() > 0)
    }
}

// Struct for Bitcoin payment method
struct Bitcoin {
    wallet_address: String,
}

// Implement the PaymentMethod trait for Bitcoin
impl PaymentMethod for Bitcoin {
    fn process_payment(&self, amount: f64) -> Result<(), String> {
        if !self.validate_details() {
            return Err("Bitcoin wallet address is invalid.".to_string());
        }
        if amount < 0.0 {
            return Err("Invalid amount, cannot process negative payment.".to_string());
        }
        println!("Processing Bitcoin payment of ${:.2}", amount);
        Ok(())
    }

    fn validate_details(&self) -> bool {
        // Validate Bitcoin wallet address length (26 to 35 characters)
        self.wallet_address.len() >= 26 && self.wallet_address.len() <= 35
    }
}

// Function to handle payments using trait objects
fn handle_payment(payment_method: &dyn PaymentMethod, amount: f64) -> Result<(), String> {
    if !payment_method.validate_details() {
        return Err("Payment details are invalid.".to_string());
    }
    payment_method.process_payment(amount)
}

fn main() {
    // Test CreditCard payment
    let credit_card = CreditCard {
        number: "1234567812345678".to_string(),
        expiry_date: "12/23".to_string(),
        cvv: "123".to_string(),
    };

    // Test Paypal payment
    let paypal = Paypal {
        email: "user@example.com".to_string(),
        password: "password123".to_string(),
    };

    // Test Bitcoin payment
    let bitcoin = Bitcoin {
        wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    };

    // Valid payments
    match handle_payment(&credit_card, 100.0) {
        Ok(_) => println!("CreditCard payment successful!"),
        Err(e) => println!("Error: {}", e),
    }

    match handle_payment(&paypal, 200.0) {
        Ok(_) => println!("Paypal payment successful!"),
        Err(e) => println!("Error: {}", e),
    }

    match handle_payment(&bitcoin, 300.0) {
        Ok(_) => println!("Bitcoin payment successful!"),
        Err(e) => println!("Error: {}", e),
    }

    // Invalid transactions for testing
    let invalid_credit_card = CreditCard {
        number: "123".to_string(),
        expiry_date: "12/23".to_string(),
        cvv: "123".to_string(),
    };

    match handle_payment(&invalid_credit_card, 100.0) {
        Ok(_) => println!("Invalid CreditCard payment successful!"),
        Err(e) => println!("Error: {}", e),
    }

    let invalid_paypal = Paypal {
        email: "invalidemail".to_string(),
        password: "password123".to_string(),
    };

    match handle_payment(&invalid_paypal, 200.0) {
        Ok(_) => println!("Invalid Paypal payment successful!"),
        Err(e) => println!("Error: {}", e),
    }

    let invalid_bitcoin = Bitcoin {
        wallet_address: "invalid_wallet".to_string(),
    };

    match handle_payment(&invalid_bitcoin, 300.0) {
        Ok(_) => println!("Invalid Bitcoin payment successful!"),
        Err(e) => println!("Error: {}", e),
    }
}
