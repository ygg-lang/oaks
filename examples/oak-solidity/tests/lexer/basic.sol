// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

/**
 * @title MyToken
 * @dev Implementation of a basic ERC20 token with additional features
 */
contract MyToken is ERC20, ERC20Burnable, Ownable, Pausable {
    using SafeMath for uint256;
    
    uint256 private constant MAX_SUPPLY = 1000000 * 10**18; // 1 million tokens
    uint256 public constant INITIAL_SUPPLY = 100000 * 10**18; // 100k tokens
    
    mapping(address => bool) public blacklisted;
    mapping(address => uint256) public lastTransferTime;
    
    uint256 public transferCooldown = 1 minutes;
    uint256 public maxTransferAmount = 10000 * 10**18; // 10k tokens
    
    event BlacklistUpdated(address indexed account, bool isBlacklisted);
    event TransferCooldownUpdated(uint256 newCooldown);
    event MaxTransferAmountUpdated(uint256 newAmount);
    
    modifier notBlacklisted(address account) {
        require(!blacklisted[account], "Account is blacklisted");
        _;
    }
    
    modifier cooldownPassed(address account) {
        require(
            block.timestamp >= lastTransferTime[account].add(transferCooldown),
            "Transfer cooldown not passed"
        );
        _;
    }
    
    constructor() ERC20("MyToken", "MTK") {
        _mint(msg.sender, INITIAL_SUPPLY);
    }
    
    function mint(address to, uint256 amount) public onlyOwner {
        require(totalSupply().add(amount) <= MAX_SUPPLY, "Exceeds max supply");
        _mint(to, amount);
    }
    
    function pause() public onlyOwner {
        _pause();
    }
    
    function unpause() public onlyOwner {
        _unpause();
    }
    
    function updateBlacklist(address account, bool isBlacklisted) public onlyOwner {
        blacklisted[account] = isBlacklisted;
        emit BlacklistUpdated(account, isBlacklisted);
    }
    
    function setTransferCooldown(uint256 newCooldown) public onlyOwner {
        transferCooldown = newCooldown;
        emit TransferCooldownUpdated(newCooldown);
    }
    
    function setMaxTransferAmount(uint256 newAmount) public onlyOwner {
        maxTransferAmount = newAmount;
        emit MaxTransferAmountUpdated(newAmount);
    }
    
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override whenNotPaused notBlacklisted(from) notBlacklisted(to) {
        if (from != address(0) && to != address(0)) {
            require(amount <= maxTransferAmount, "Transfer amount exceeds limit");
        }
        super._beforeTokenTransfer(from, to, amount);
    }
    
    function transfer(address to, uint256 amount) 
        public 
        override 
        cooldownPassed(msg.sender) 
        returns (bool) 
    {
        lastTransferTime[msg.sender] = block.timestamp;
        return super.transfer(to, amount);
    }
    
    function transferFrom(address from, address to, uint256 amount) 
        public 
        override 
        cooldownPassed(from) 
        returns (bool) 
    {
        lastTransferTime[from] = block.timestamp;
        return super.transferFrom(from, to, amount);
    }
}

/**
 * @title TokenSale
 * @dev A simple token sale contract
 */
contract TokenSale is ReentrancyGuard, Ownable {
    using SafeMath for uint256;
    
    MyToken public token;
    uint256 public rate; // tokens per wei
    uint256 public weiRaised;
    uint256 public cap;
    uint256 public openingTime;
    uint256 public closingTime;
    
    mapping(address => uint256) public contributions;
    mapping(address => bool) public whitelist;
    
    event TokensPurchased(address indexed purchaser, address indexed beneficiary, uint256 value, uint256 amount);
    event WhitelistUpdated(address indexed account, bool isWhitelisted);
    event RateUpdated(uint256 newRate);
    
    modifier onlyWhileOpen {
        require(isOpen(), "Sale is not open");
        _;
    }
    
    modifier onlyWhitelisted(address account) {
        require(whitelist[account], "Account not whitelisted");
        _;
    }
    
    constructor(
        uint256 _rate,
        address payable _wallet,
        MyToken _token,
        uint256 _cap,
        uint256 _openingTime,
        uint256 _closingTime
    ) {
        require(_rate > 0, "Rate is 0");
        require(_wallet != address(0), "Wallet is zero address");
        require(address(_token) != address(0), "Token is zero address");
        require(_cap > 0, "Cap is 0");
        require(_openingTime >= block.timestamp, "Opening time is before current time");
        require(_closingTime > _openingTime, "Closing time is before opening time");
        
        rate = _rate;
        token = _token;
        cap = _cap;
        openingTime = _openingTime;
        closingTime = _closingTime;
    }
    
    function buyTokens(address beneficiary) public payable nonReentrant onlyWhileOpen onlyWhitelisted(beneficiary) {
        uint256 weiAmount = msg.value;
        _preValidatePurchase(beneficiary, weiAmount);
        
        uint256 tokens = _getTokenAmount(weiAmount);
        
        weiRaised = weiRaised.add(weiAmount);
        contributions[beneficiary] = contributions[beneficiary].add(weiAmount);
        
        _processPurchase(beneficiary, tokens);
        emit TokensPurchased(msg.sender, beneficiary, weiAmount, tokens);
        
        _forwardFunds();
    }
    
    function _preValidatePurchase(address beneficiary, uint256 weiAmount) internal view {
        require(beneficiary != address(0), "Beneficiary is zero address");
        require(weiAmount != 0, "Wei amount is 0");
        require(weiRaised.add(weiAmount) <= cap, "Cap exceeded");
    }
    
    function _processPurchase(address beneficiary, uint256 tokenAmount) internal {
        token.transfer(beneficiary, tokenAmount);
    }
    
    function _getTokenAmount(uint256 weiAmount) internal view returns (uint256) {
        return weiAmount.mul(rate);
    }
    
    function _forwardFunds() internal {
        payable(owner()).transfer(msg.value);
    }
    
    function isOpen() public view returns (bool) {
        return block.timestamp >= openingTime && block.timestamp <= closingTime;
    }
    
    function hasClosed() public view returns (bool) {
        return block.timestamp > closingTime;
    }
    
    function capReached() public view returns (bool) {
        return weiRaised >= cap;
    }
    
    function updateWhitelist(address account, bool isWhitelisted) public onlyOwner {
        whitelist[account] = isWhitelisted;
        emit WhitelistUpdated(account, isWhitelisted);
    }
    
    function updateRate(uint256 newRate) public onlyOwner {
        require(newRate > 0, "Rate must be greater than 0");
        rate = newRate;
        emit RateUpdated(newRate);
    }
    
    function withdrawTokens() public onlyOwner {
        require(hasClosed(), "Sale has not closed");
        uint256 balance = token.balanceOf(address(this));
        token.transfer(owner(), balance);
    }
    
    receive() external payable {
        buyTokens(msg.sender);
    }
}

/**
 * @title MultiSigWallet
 * @dev A simple multi-signature wallet
 */
contract MultiSigWallet {
    using SafeMath for uint256;
    
    event Deposit(address indexed sender, uint256 amount, uint256 balance);
    event SubmitTransaction(
        address indexed owner,
        uint256 indexed txIndex,
        address indexed to,
        uint256 value,
        bytes data
    );
    event ConfirmTransaction(address indexed owner, uint256 indexed txIndex);
    event RevokeConfirmation(address indexed owner, uint256 indexed txIndex);
    event ExecuteTransaction(address indexed owner, uint256 indexed txIndex);
    
    address[] public owners;
    mapping(address => bool) public isOwner;
    uint256 public numConfirmationsRequired;
    
    struct Transaction {
        address to;
        uint256 value;
        bytes data;
        bool executed;
        uint256 numConfirmations;
    }
    
    mapping(uint256 => mapping(address => bool)) public isConfirmed;
    Transaction[] public transactions;
    
    modifier onlyOwner() {
        require(isOwner[msg.sender], "Not owner");
        _;
    }
    
    modifier txExists(uint256 _txIndex) {
        require(_txIndex < transactions.length, "Transaction does not exist");
        _;
    }
    
    modifier notExecuted(uint256 _txIndex) {
        require(!transactions[_txIndex].executed, "Transaction already executed");
        _;
    }
    
    modifier notConfirmed(uint256 _txIndex) {
        require(!isConfirmed[_txIndex][msg.sender], "Transaction already confirmed");
        _;
    }
    
    constructor(address[] memory _owners, uint256 _numConfirmationsRequired) {
        require(_owners.length > 0, "Owners required");
        require(
            _numConfirmationsRequired > 0 && _numConfirmationsRequired <= _owners.length,
            "Invalid number of required confirmations"
        );
        
        for (uint256 i = 0; i < _owners.length; i++) {
            address owner = _owners[i];
            require(owner != address(0), "Invalid owner");
            require(!isOwner[owner], "Owner not unique");
            
            isOwner[owner] = true;
            owners.push(owner);
        }
        
        numConfirmationsRequired = _numConfirmationsRequired;
    }
    
    receive() external payable {
        emit Deposit(msg.sender, msg.value, address(this).balance);
    }
    
    function submitTransaction(
        address _to,
        uint256 _value,
        bytes memory _data
    ) public onlyOwner {
        uint256 txIndex = transactions.length;
        
        transactions.push(Transaction({
            to: _to,
            value: _value,
            data: _data,
            executed: false,
            numConfirmations: 0
        }));
        
        emit SubmitTransaction(msg.sender, txIndex, _to, _value, _data);
    }
    
    function confirmTransaction(uint256 _txIndex)
        public
        onlyOwner
        txExists(_txIndex)
        notExecuted(_txIndex)
        notConfirmed(_txIndex)
    {
        Transaction storage transaction = transactions[_txIndex];
        transaction.numConfirmations = transaction.numConfirmations.add(1);
        isConfirmed[_txIndex][msg.sender] = true;
        
        emit ConfirmTransaction(msg.sender, _txIndex);
    }
    
    function executeTransaction(uint256 _txIndex)
        public
        onlyOwner
        txExists(_txIndex)
        notExecuted(_txIndex)
    {
        Transaction storage transaction = transactions[_txIndex];
        
        require(
            transaction.numConfirmations >= numConfirmationsRequired,
            "Cannot execute transaction"
        );
        
        transaction.executed = true;
        
        (bool success, ) = transaction.to.call{value: transaction.value}(transaction.data);
        require(success, "Transaction failed");
        
        emit ExecuteTransaction(msg.sender, _txIndex);
    }
    
    function revokeConfirmation(uint256 _txIndex)
        public
        onlyOwner
        txExists(_txIndex)
        notExecuted(_txIndex)
    {
        Transaction storage transaction = transactions[_txIndex];
        
        require(isConfirmed[_txIndex][msg.sender], "Transaction not confirmed");
        
        transaction.numConfirmations = transaction.numConfirmations.sub(1);
        isConfirmed[_txIndex][msg.sender] = false;
        
        emit RevokeConfirmation(msg.sender, _txIndex);
    }
    
    function getOwners() public view returns (address[] memory) {
        return owners;
    }
    
    function getTransactionCount() public view returns (uint256) {
        return transactions.length;
    }
    
    function getTransaction(uint256 _txIndex)
        public
        view
        returns (
            address to,
            uint256 value,
            bytes memory data,
            bool executed,
            uint256 numConfirmations
        )
    {
        Transaction storage transaction = transactions[_txIndex];
        
        return (
            transaction.to,
            transaction.value,
            transaction.data,
            transaction.executed,
            transaction.numConfirmations
        );
    }
}

/**
 * @title SimpleStorage
 * @dev Store and retrieve value in a variable
 */
contract SimpleStorage {
    uint256 storedData;
    
    event DataStored(uint256 data);
    
    function set(uint256 x) public {
        storedData = x;
        emit DataStored(x);
    }
    
    function get() public view returns (uint256) {
        return storedData;
    }
}

/**
 * @title Voting
 * @dev A simple voting contract
 */
contract Voting {
    struct Voter {
        uint256 weight;
        bool voted;
        address delegate;
        uint256 vote;
    }
    
    struct Proposal {
        bytes32 name;
        uint256 voteCount;
    }
    
    address public chairperson;
    mapping(address => Voter) public voters;
    Proposal[] public proposals;
    
    event VoteCast(address indexed voter, uint256 proposal);
    event DelegateVote(address indexed voter, address indexed delegate);
    
    constructor(bytes32[] memory proposalNames) {
        chairperson = msg.sender;
        voters[chairperson].weight = 1;
        
        for (uint256 i = 0; i < proposalNames.length; i++) {
            proposals.push(Proposal({
                name: proposalNames[i],
                voteCount: 0
            }));
        }
    }
    
    function giveRightToVote(address voter) public {
        require(msg.sender == chairperson, "Only chairperson can give right to vote");
        require(!voters[voter].voted, "The voter already voted");
        require(voters[voter].weight == 0, "Voter already has voting rights");
        voters[voter].weight = 1;
    }
    
    function delegate(address to) public {
        Voter storage sender = voters[msg.sender];
        require(!sender.voted, "You already voted");
        require(to != msg.sender, "Self-delegation is disallowed");
        
        while (voters[to].delegate != address(0)) {
            to = voters[to].delegate;
            require(to != msg.sender, "Found loop in delegation");
        }
        
        sender.voted = true;
        sender.delegate = to;
        Voter storage delegate_ = voters[to];
        
        if (delegate_.voted) {
            proposals[delegate_.vote].voteCount += sender.weight;
        } else {
            delegate_.weight += sender.weight;
        }
        
        emit DelegateVote(msg.sender, to);
    }
    
    function vote(uint256 proposal) public {
        Voter storage sender = voters[msg.sender];
        require(sender.weight != 0, "Has no right to vote");
        require(!sender.voted, "Already voted");
        
        sender.voted = true;
        sender.vote = proposal;
        proposals[proposal].voteCount += sender.weight;
        
        emit VoteCast(msg.sender, proposal);
    }
    
    function winningProposal() public view returns (uint256 winningProposal_) {
        uint256 winningVoteCount = 0;
        for (uint256 p = 0; p < proposals.length; p++) {
            if (proposals[p].voteCount > winningVoteCount) {
                winningVoteCount = proposals[p].voteCount;
                winningProposal_ = p;
            }
        }
    }
    
    function winnerName() public view returns (bytes32 winnerName_) {
        winnerName_ = proposals[winningProposal()].name;
    }
}

/**
 * @title Auction
 * @dev A simple auction contract
 */
contract Auction {
    address payable public beneficiary;
    uint256 public auctionEndTime;
    
    address public highestBidder;
    uint256 public highestBid;
    
    mapping(address => uint256) pendingReturns;
    
    bool ended;
    
    event HighestBidIncreased(address bidder, uint256 amount);
    event AuctionEnded(address winner, uint256 amount);
    
    constructor(uint256 _biddingTime, address payable _beneficiary) {
        beneficiary = _beneficiary;
        auctionEndTime = block.timestamp + _biddingTime;
    }
    
    function bid() public payable {
        require(block.timestamp <= auctionEndTime, "Auction already ended");
        require(msg.value > highestBid, "There already is a higher bid");
        
        if (highestBid != 0) {
            pendingReturns[highestBidder] += highestBid;
        }
        
        highestBidder = msg.sender;
        highestBid = msg.value;
        emit HighestBidIncreased(msg.sender, msg.value);
    }
    
    function withdraw() public returns (bool) {
        uint256 amount = pendingReturns[msg.sender];
        if (amount > 0) {
            pendingReturns[msg.sender] = 0;
            
            if (!payable(msg.sender).send(amount)) {
                pendingReturns[msg.sender] = amount;
                return false;
            }
        }
        return true;
    }
    
    function auctionEnd() public {
        require(block.timestamp >= auctionEndTime, "Auction not yet ended");
        require(!ended, "auctionEnd has already been called");
        
        ended = true;
        emit AuctionEnded(highestBidder, highestBid);
        
        beneficiary.transfer(highestBid);
    }
}

/**
 * @title Library example
 */
library SafeMathLib {
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        uint256 c = a + b;
        require(c >= a, "SafeMath: addition overflow");
        return c;
    }
    
    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        require(b <= a, "SafeMath: subtraction overflow");
        return a - b;
    }
    
    function mul(uint256 a, uint256 b) internal pure returns (uint256) {
        if (a == 0) return 0;
        uint256 c = a * b;
        require(c / a == b, "SafeMath: multiplication overflow");
        return c;
    }
    
    function div(uint256 a, uint256 b) internal pure returns (uint256) {
        require(b > 0, "SafeMath: division by zero");
        return a / b;
    }
}

/**
 * @title Interface example
 */
interface IERC20Extended {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address recipient, uint256 amount) external returns (bool);
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    
    // Extended functions
    function mint(address to, uint256 amount) external;
    function burn(uint256 amount) external;
    function pause() external;
    function unpause() external;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

/**
 * @title Abstract contract example
 */
abstract contract Token {
    string public name;
    string public symbol;
    uint8 public decimals;
    
    constructor(string memory _name, string memory _symbol, uint8 _decimals) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
    }
    
    function totalSupply() public view virtual returns (uint256);
    function balanceOf(address account) public view virtual returns (uint256);
    function transfer(address to, uint256 amount) public virtual returns (bool);
}

/**
 * @title Enum example
 */
contract StateMachine {
    enum State { Waiting, Ready, Active, Inactive }
    State public state;
    
    event StateChanged(State newState);
    
    modifier inState(State _state) {
        require(state == _state, "Invalid state");
        _;
    }
    
    function activate() public inState(State.Ready) {
        state = State.Active;
        emit StateChanged(state);
    }
    
    function deactivate() public inState(State.Active) {
        state = State.Inactive;
        emit StateChanged(state);
    }
    
    function reset() public {
        state = State.Waiting;
        emit StateChanged(state);
    }
}