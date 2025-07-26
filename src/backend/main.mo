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
        #Commission;
    };
    
    public type WithdrawalStatus = {
        #Pending;
        #Approved;
        #Rejected;
    };

    public type WithdrawalRequest = {
        id: Text;
        userId: Text;
        amount: Nat;
        timestamp: Int;
        status: WithdrawalStatus;
        code: Text;
        agentId: ?Text; // Agent who processed the request
    };

    public type AgentStats = {
        totalTransactions: Nat;
        commissionsEarned: Nat;
        lastActivity: Int;
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
    private stable var withdrawalEntries : [(Text, WithdrawalRequest)] = [];
    private stable var agentStatsEntries : [(Text, AgentStats)] = [];
    
    private var users = HashMap.HashMap<Text, User>(0, Text.equal, Text.hash);
    private var transactions = HashMap.HashMap<Text, Transaction>(0, Text.equal, Text.hash);
    private var withdrawalRequests = HashMap.HashMap<Text, WithdrawalRequest>(0, Text.equal, Text.hash);
    private var agentStats = HashMap.HashMap<Text, AgentStats>(0, Text.equal, Text.hash);
    
    // Constants
    private let COMMISSION_RATE : Nat = 2; // 2% commission for agents
    
    // Initialize from stable storage
    system func preupgrade() {
        userEntries := Iter.toArray(users.entries());
        transactionEntries := Iter.toArray(transactions.entries());
        withdrawalEntries := Iter.toArray(withdrawalRequests.entries());
        agentStatsEntries := Iter.toArray(agentStats.entries());
    };
    
    system func postupgrade() {
        users := HashMap.fromIter<Text, User>(userEntries.vals(), userEntries.size(), Text.equal, Text.hash);
        transactions := HashMap.fromIter<Text, Transaction>(transactionEntries.vals(), transactionEntries.size(), Text.equal, Text.hash);
        withdrawalRequests := HashMap.fromIter<Text, WithdrawalRequest>(withdrawalEntries.vals(), withdrawalEntries.size(), Text.equal, Text.hash);
        agentStats := HashMap.fromIter<Text, AgentStats>(agentStatsEntries.vals(), agentStatsEntries.size(), Text.equal, Text.hash);
    };

    // Utility functions
    private func generateId() : Text {
        let timestamp = Time.now();
        Int.toText(timestamp)
    };

    // Agent specific functions
    public func getPendingWithdrawals(agentPhoneNumber: Text) : async Result.Result<[WithdrawalRequest], Text> {
        // Verify agent
        switch(users.get(agentPhoneNumber)) {
            case(?user) {
                switch(user.userType) {
                    case(#Agent) {
                        let pending = Iter.toArray(withdrawalRequests.vals());
                        let filtered = Array.filter(pending, func (w: WithdrawalRequest) : Bool {
                            w.status == #Pending
                        });
                        #ok(filtered)
                    };
                    case(_) { #err("Not authorized. Only agents can view pending withdrawals.") };
                }
            };
            case null { #err("Agent not found") };
        }
    };

    public func getAgentStats(agentPhoneNumber: Text) : async Result.Result<AgentStats, Text> {
        switch(agentStats.get(agentPhoneNumber)) {
            case(?stats) { #ok(stats) };
            case null { 
                let newStats = {
                    totalTransactions = 0;
                    commissionsEarned = 0;
                    lastActivity = Time.now();
                };
                agentStats.put(agentPhoneNumber, newStats);
                #ok(newStats)
            };
        }
    };

    public func approveWithdrawal(agentPhoneNumber: Text, withdrawalId: Text, pin: Text) : async Result.Result<WithdrawalRequest, Text> {
        // Verify agent PIN
        if (not validatePin(agentPhoneNumber, pin)) {
            return #err("Invalid PIN");
        };

        // Verify agent
        switch(users.get(agentPhoneNumber)) {
            case(?agent) {
                switch(agent.userType) {
                    case(#Agent) {
                        switch(withdrawalRequests.get(withdrawalId)) {
                            case(?request) {
                                if (request.status != #Pending) {
                                    return #err("Withdrawal request is not pending");
                                };

                                // Get user
                                switch(users.get(request.userId)) {
                                    case(?user) {
                                        if (user.balance < request.amount) {
                                            return #err("Insufficient user balance");
                                        };

                                        // Calculate commission
                                        let commission = request.amount * COMMISSION_RATE / 100;

                                        // Update user balance
                                        let updatedUser = {
                                            user with balance = user.balance - request.amount
                                        };
                                        users.put(request.userId, updatedUser);

                                        // Update agent balance and stats
                                        let updatedAgent = {
                                            agent with balance = agent.balance + commission
                                        };
                                        users.put(agentPhoneNumber, updatedAgent);

                                        // Update agent stats
                                        switch(agentStats.get(agentPhoneNumber)) {
                                            case(?stats) {
                                                let updatedStats = {
                                                    totalTransactions = stats.totalTransactions + 1;
                                                    commissionsEarned = stats.commissionsEarned + commission;
                                                    lastActivity = Time.now();
                                                };
                                                agentStats.put(agentPhoneNumber, updatedStats);
                                            };
                                            case null {
                                                let newStats = {
                                                    totalTransactions = 1;
                                                    commissionsEarned = commission;
                                                    lastActivity = Time.now();
                                                };
                                                agentStats.put(agentPhoneNumber, newStats);
                                            };
                                        };

                                        // Update withdrawal request
                                        let updatedRequest = {
                                            request with
                                            status = #Approved;
                                            agentId = ?agentPhoneNumber;
                                        };
                                        withdrawalRequests.put(withdrawalId, updatedRequest);

                                        // Record transactions
                                        let withdrawalTx : Transaction = {
                                            id = generateId();
                                            from = request.userId;
                                            to = agentPhoneNumber;
                                            amount = request.amount;
                                            transactionType = #Withdraw;
                                            timestamp = Time.now();
                                        };
                                        transactions.put(withdrawalTx.id, withdrawalTx);

                                        let commissionTx : Transaction = {
                                            id = generateId();
                                            from = "system";
                                            to = agentPhoneNumber;
                                            amount = commission;
                                            transactionType = #Commission;
                                            timestamp = Time.now();
                                        };
                                        transactions.put(commissionTx.id, commissionTx);

                                        #ok(updatedRequest)
                                    };
                                    case null { #err("User not found") };
                                }
                            };
                            case null { #err("Withdrawal request not found") };
                        }
                    };
                    case(_) { #err("Not authorized. Only agents can approve withdrawals.") };
                }
            };
            case null { #err("Agent not found") };
        }
    };

    public func getAgentTransactions(agentPhoneNumber: Text) : async Result.Result<[Transaction], Text> {
        // Verify agent
        switch(users.get(agentPhoneNumber)) {
            case(?user) {
                switch(user.userType) {
                    case(#Agent) {
                        let allTx = Iter.toArray(transactions.vals());
                        let agentTx = Array.filter(allTx, func (tx: Transaction) : Bool {
                            tx.from == agentPhoneNumber or tx.to == agentPhoneNumber
                        });
                        #ok(agentTx)
                    };
                    case(_) { #err("Not authorized. Only agents can view their transactions.") };
                }
            };
            case null { #err("Agent not found") };
        }
    };

    // // Modified register user function to support agent registration
    // public func registerUser(phoneNumber: Text, pin: Text, userType: UserType) : async Result.Result<User, Text> {
    //     switch(users.get(phoneNumber)) {
    //         case(?_) { #err("User already exists") };
    //         case null {
    

    
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
    public func initiateWithdrawal(phoneNumber: Text, amount: Nat, pin: Text) : async Result.Result<WithdrawalRequest, Text> {
        if (not validatePin(phoneNumber, pin)) {
            return #err("Invalid PIN");
        };
        
        switch(users.get(phoneNumber)) {
            case(?user) {
                if (user.balance < amount) {
                    return #err("Insufficient balance");
                };
                
                // Generate withdrawal code and request ID
                let withdrawalCode = "WD" # Int.toText(Time.now() % 999999);
                let requestId = generateId();
                
                // Create withdrawal request
                let request : WithdrawalRequest = {
                    id = requestId;
                    userId = phoneNumber;
                    amount = amount;
                    timestamp = Time.now();
                    status = #Pending;
                    code = withdrawalCode;
                    agentId = null;
                };
                
                // Store the withdrawal request
                withdrawalRequests.put(requestId, request);
                
                #ok(request)
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
                // Verify this is a regular user, not an agent
                switch(user.userType) {
                    case(#User) {
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

                        // Update only user balance
                        let updatedUser : User = {
                            user with balance = user.balance + amount;
                        };
                        users.put(phoneNumber, updatedUser);

                        // Store transaction
                        transactions.put(transactionId, transaction);
                        #ok(transaction)
                    };
                    case(#Agent) {
                        #err("Agents cannot make direct deposits. Use withdrawal approval for agent transactions.")
                    };
                }
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