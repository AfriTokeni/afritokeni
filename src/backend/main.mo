import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Time "mo:base/Time";
import HashMap "mo:base/HashMap";
import Result "mo:base/Result";
import Nat "mo:base/Nat";
import Array "mo:base/Array";
import Debug "mo:base/Debug";
import Iter "mo:base/Iter";
import Int "mo:base/Int";

actor MoneyTransferCanister {
    
    // Types
    public type UserType = {
        #User;
        #Agent;
    };
    
    public type User = {
        id: Principal;
        phoneNumber: Text;
        userType: UserType;
        balance: Nat;
        pin: Text; // Hashed
        isActive: Bool;
        createdAt: Int;
    };
    
    public type PaymentMethod = {
        #MTN;
        #Airtel;
    };

    public type TransactionType = {
        #Send;
        #Receive;
        #Withdraw;
        #Deposit : PaymentMethod;
    };
    
    public type Transaction = {
        id: Text;
        from: Text; // Phone number
        to: Text;   // Phone number
        amount: Nat;
        transactionType: TransactionType;
        timestamp: Int;
    };
    
    public type USSDResponse = {
        message: Text;
        shouldEnd: Bool;
    };
    
    public type BalanceInfo = {
        balance: Nat;
        lastTransaction: ?Transaction;
    };
    
    // Storage
    private stable var userEntries : [(Text, User)] = [];
    private stable var transactionEntries : [(Text, Transaction)] = [];
    private var users = HashMap.HashMap<Text, User>(0, Text.equal, Text.hash);
    private var transactions = HashMap.HashMap<Text, Transaction>(0, Text.equal, Text.hash);
    
    // Initialize from stable storage
    system func preupgrade() {
        userEntries := Iter.toArray(users.entries());
        transactionEntries := Iter.toArray(transactions.entries());
    };
    
    system func postupgrade() {
        users := HashMap.fromIter<Text, User>(userEntries.vals(), userEntries.size(), Text.equal, Text.hash);
        transactions := HashMap.fromIter<Text, Transaction>(transactionEntries.vals(), transactionEntries.size(), Text.equal, Text.hash);
    };
    
    // Utility functions
    private func generateId() : Text {
        let timestamp = Time.now();
        Int.toText(timestamp)
    };
    
    private func validatePin(phoneNumber: Text, pin: Text) : Bool {
        switch(users.get(phoneNumber)) {
            case(?user) { user.pin == pin }; // In production, hash and compare
            case null { false };
        }
    };
    
    // Public functions for USSD operations
    
    // Register new user
    public func registerUser(phoneNumber: Text, pin: Text, userType: UserType) : async Result.Result<User, Text> {
        switch(users.get(phoneNumber)) {
            case(?existingUser) {
                #err("User already exists")
            };
            case null {
                let newUser : User = {
                    id = Principal.fromText("aaaaa-aa");
                    phoneNumber = phoneNumber;
                    userType = userType;
                    balance = 0;
                    pin = pin; // Hash this in production
                    isActive = true;
                    createdAt = Time.now();
                };
                users.put(phoneNumber, newUser);
                #ok(newUser)
            };
        }
    };
    
    // Check balance with PIN verification
    public func checkBalance(phoneNumber: Text, pin: Text) : async Result.Result<BalanceInfo, Text> {
        if (not validatePin(phoneNumber, pin)) {
            return #err("Invalid PIN");
        };
        
        switch(users.get(phoneNumber)) {
            case(?user) {
                // Get last transaction
                let userTransactions = transactions.vals() 
                    |> Iter.toArray(_)
                    |> Array.filter(_, func(t: Transaction) : Bool { t.from == phoneNumber or t.to == phoneNumber });
                
                let lastTransaction = if (userTransactions.size() > 0) {
                    ?userTransactions[userTransactions.size() - 1]
                } else { null };
                
                let balanceInfo : BalanceInfo = {
                    balance = user.balance;
                    lastTransaction = lastTransaction;
                };
                #ok(balanceInfo)
            };
            case null {
                #err("User not found")
            };
        }
    };
    
    // Send money
    public func sendMoney(from: Text, to: Text, amount: Nat, pin: Text) : async Result.Result<Transaction, Text> {
        if (not validatePin(from, pin)) {
            return #err("Invalid PIN");
        };
        
        switch(users.get(from)) {
            case(?sender) {
                if (sender.balance < amount) {
                    return #err("Insufficient balance");
                };
                
                // Create transaction
                let transactionId = generateId();
                let transaction : Transaction = {
                    id = transactionId;
                    from = from;
                    to = to;
                    amount = amount;
                    transactionType = #Send;
                    timestamp = Time.now();
                };
                
                // Update sender balance
                let updatedSender : User = {
                    sender with balance = sender.balance - amount;
                };
                users.put(from, updatedSender);
                
                // Update recipient balance if they exist
                switch(users.get(to)) {
                    case(?recipient) {
                        let updatedRecipient : User = {
                            recipient with balance = recipient.balance + amount;
                        };
                        users.put(to, updatedRecipient);
                    };
                    case null {
                        // Recipient doesn't exist - handle pending transaction
                        Debug.print("Recipient not found: " # to);
                    };
                };
                
                // Store transaction
                transactions.put(transactionId, transaction);
                #ok(transaction)
            };
            case null {
                #err("Sender not found")
            };
        }
    };
    
    // Process withdrawal (generate withdrawal code)
    public func initiateWithdrawal(phoneNumber: Text, amount: Nat, pin: Text) : async Result.Result<Text, Text> {
        if (not validatePin(phoneNumber, pin)) {
            return #err("Invalid PIN");
        };
        
        switch(users.get(phoneNumber)) {
            case(?user) {
                if (user.balance < amount) {
                    return #err("Insufficient balance");
                };
                
                // Generate withdrawal code (simplified)
                let withdrawalCode = "WD" # Int.toText(Time.now() % 999999);
                
                // In production, store this code with expiry
                #ok(withdrawalCode)
            };
            case null {
                #err("User not found")
            };
        }
    };
    
    // Deposit money
    public func depositMoney(phoneNumber: Text, amount: Nat, paymentMethod: PaymentMethod, pin: Text) : async Result.Result<Transaction, Text> {
        if (not validatePin(phoneNumber, pin)) {
            return #err("Invalid PIN");
        };

        switch(users.get(phoneNumber)) {
            case(?user) {
                // Create transaction
                let transactionId = generateId();
                let transaction : Transaction = {
                    id = transactionId;
                    from = phoneNumber;
                    to = phoneNumber; // Self deposit
                    amount = amount;
                    transactionType = #Deposit(paymentMethod);
                    timestamp = Time.now();
                };

                // Update user balance
                let updatedUser : User = {
                    user with balance = user.balance + amount;
                };
                users.put(phoneNumber, updatedUser);

                // Store transaction
                transactions.put(transactionId, transaction);
                #ok(transaction)
            };
            case null {
                #err("User not found")
            };
        }
    };

    // Get user by phone number
    public query func getUser(phoneNumber: Text) : async ?User {
        users.get(phoneNumber)
    };
}