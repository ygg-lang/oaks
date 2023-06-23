# Nix Expression Language Test File - Comprehensive Syntax Coverage
# This file tests various Nix syntax elements for lexer testing

# Basic values and literals
let
  # Primitive types
  intValue = 42;
  floatValue = 3.14159;
  stringValue = "Hello, Nix!";
  boolTrue = true;
  boolFalse = false;
  nullValue = null;
  
  # String interpolation
  name = "World";
  greeting = "Hello, ${name}!";
  multiLineString = ''
    This is a multi-line string
    with ${name} interpolation
    and preserved whitespace.
  '';
  
  # Paths
  absolutePath = /etc/nixos/configuration.nix;
  relativePath = ./default.nix;
  homePath = ~/.config/nixpkgs/config.nix;
  
  # URIs
  httpUrl = https://github.com/NixOS/nixpkgs;
  gitUrl = git+https://github.com/NixOS/nixpkgs.git;
  
  # Lists
  emptyList = [];
  numberList = [ 1 2 3 4 5 ];
  mixedList = [ 1 "hello" true null ];
  nestedList = [ [ 1 2 ] [ 3 4 ] [ 5 6 ] ];
  
  # Attribute sets (objects)
  emptySet = {};
  simpleSet = { x = 1; y = 2; };
  nestedSet = {
    level1 = {
      level2 = {
        value = "deep";
      };
    };
  };
  
  # Recursive attribute sets
  recursiveSet = rec {
    a = 1;
    b = a + 1;
    c = b * 2;
  };
  
  # Attribute set with string keys
  dynamicKeys = {
    "key with spaces" = "value1";
    "${name}" = "dynamic key";
    "123" = "numeric key";
  };
  
  # Functions
  simpleFunction = x: x + 1;
  multiArgFunction = x: y: x + y;
  functionWithDefault = { x ? 10, y ? 20 }: x + y;
  functionWithEllipsis = { x, y, ... }: x + y;
  
  # Pattern matching in functions
  destructuringFunction = { name, age, ... }↯person: 
    "Person ${name} is ${toString age} years old";
  
  # List pattern matching
  listFunction = [ head ] ++ tail: head;
  
  # Conditional expressions
  conditionalValue = if true then "yes" else "no";
  nestedConditional = if false then "a" else if true then "b" else "c";
  
  # Let expressions
  letExpression = let
    x = 10;
    y = 20;
  in x + y;
  
  # Nested let expressions
  nestedLet = let
    outer = 5;
  in let
    inner = 10;
  in outer + inner;
  
  # With expressions
  withExpression = with { a = 1; b = 2; }; a + b;
  
  # Inherit expressions
  inheritExample = let
    base = { x = 1; y = 2; };
  in {
    inherit (base) x y;
    z = 3;
  };
  
  # Assert expressions
  assertExample = assert 1 + 1 == 2; "assertion passed";
  
  # Operators
  arithmetic = {
    addition = 1 + 2;
    subtraction = 5 - 3;
    multiplication = 4 * 6;
    division = 10 / 2;
    modulo = 7 % 3;
  };
  
  comparison = {
    equal = 1 == 1;
    notEqual = 1 != 2;
    lessThan = 1 < 2;
    lessEqual = 1 <= 1;
    greaterThan = 2 > 1;
    greaterEqual = 2 >= 2;
  };
  
  logical = {
    and = true && true;
    or = false || true;
    not = !false;
    implication = false -> true;
  };
  
  # List operations
  listOps = {
    concatenation = [ 1 2 ] ++ [ 3 4 ];
    hasElement = builtins.elem 2 [ 1 2 3 ];
    length = builtins.length [ 1 2 3 4 5 ];
    head = builtins.head [ 1 2 3 ];
    tail = builtins.tail [ 1 2 3 ];
  };
  
  # Attribute set operations
  setOps = {
    hasAttribute = { x = 1; } ? x;
    getAttr = { x = 1; }.x;
    merge = { a = 1; } // { b = 2; };
    keys = builtins.attrNames { x = 1; y = 2; };
    values = builtins.attrValues { x = 1; y = 2; };
  };
  
  # String operations
  stringOps = {
    concatenation = "Hello" + " " + "World";
    length = builtins.stringLength "hello";
    substring = builtins.substring 1 3 "hello";
    split = builtins.split "," "a,b,c";
    match = builtins.match "([0-9]+)" "123";
  };
  
  # Type checking
  typeChecks = {
    isString = builtins.isString "hello";
    isInt = builtins.isInt 42;
    isFloat = builtins.isFloat 3.14;
    isBool = builtins.isBool true;
    isList = builtins.isList [ 1 2 3 ];
    isAttrs = builtins.isAttrs { x = 1; };
    isFunction = builtins.isFunction (x: x);
    isNull = builtins.isNull null;
    isPath = builtins.isPath /etc;
  };
  
  # Built-in functions
  builtinFunctions = {
    # Math functions
    add = builtins.add 1 2;
    sub = builtins.sub 5 3;
    mul = builtins.mul 4 6;
    div = builtins.div 10 2;
    
    # List functions
    map = builtins.map (x: x * 2) [ 1 2 3 ];
    filter = builtins.filter (x: x > 2) [ 1 2 3 4 5 ];
    foldl = builtins.foldl' (acc: x: acc + x) 0 [ 1 2 3 4 5 ];
    sort = builtins.sort (a: b: a < b) [ 3 1 4 1 5 ];
    reverse = builtins.reverse [ 1 2 3 ];
    
    # String functions
    toString = builtins.toString 42;
    toJSON = builtins.toJSON { x = 1; y = [ 1 2 3 ]; };
    fromJSON = builtins.fromJSON ''{"x": 1, "y": [1, 2, 3]}'';
    
    # Path functions
    baseNameOf = builtins.baseNameOf /path/to/file.txt;
    dirOf = builtins.dirOf /path/to/file.txt;
    pathExists = builtins.pathExists /etc;
    
    # Hash functions
    hashString = builtins.hashString "sha256" "hello world";
    hashFile = builtins.hashFile "sha256" /etc/passwd;
  };
  
  # File operations
  fileOps = {
    readFile = builtins.readFile /etc/hostname;
    readDir = builtins.readDir /etc;
    fileExists = builtins.pathExists /etc/passwd;
  };
  
  # Derivations (packages)
  simpleDerivation = derivation {
    name = "hello";
    system = "x86_64-linux";
    builder = "/bin/sh";
    args = [ "-c" "echo hello > $out" ];
  };
  
  # Package with dependencies
  packageWithDeps = stdenv.mkDerivation {
    pname = "my-package";
    version = "1.0.0";
    
    src = fetchurl {
      url = "https://example.com/package.tar.gz";
      sha256 = "0000000000000000000000000000000000000000000000000000000000000000";
    };
    
    buildInputs = [ gcc make ];
    nativeBuildInputs = [ pkg-config ];
    
    configurePhase = ''
      ./configure --prefix=$out
    '';
    
    buildPhase = ''
      make -j$NIX_BUILD_CORES
    '';
    
    installPhase = ''
      make install
    '';
    
    meta = with lib; {
      description = "A sample package";
      homepage = "https://example.com";
      license = licenses.mit;
      maintainers = with maintainers; [ ];
      platforms = platforms.linux;
    };
  };
  
  # Overlays
  overlay = self: super: {
    myPackage = super.callPackage ./my-package.nix {};
    python3 = super.python3.override {
      packageOverrides = python-self: python-super: {
        myPythonPackage = python-super.buildPythonPackage {
          pname = "my-python-package";
          version = "1.0.0";
          src = ./src;
        };
      };
    };
  };
  
  # Configuration examples
  nixosConfig = {
    # System configuration
    system.stateVersion = "23.05";
    
    # Boot configuration
    boot = {
      loader = {
        systemd-boot.enable = true;
        efi.canTouchEfiVariables = true;
      };
      kernelPackages = pkgs.linuxPackages_latest;
    };
    
    # Networking
    networking = {
      hostName = "nixos-machine";
      networkmanager.enable = true;
      firewall = {
        enable = true;
        allowedTCPPorts = [ 22 80 443 ];
      };
    };
    
    # Services
    services = {
      openssh = {
        enable = true;
        settings = {
          PasswordAuthentication = false;
          KbdInteractiveAuthentication = false;
        };
      };
      
      nginx = {
        enable = true;
        virtualHosts."example.com" = {
          enableACME = true;
          forceSSL = true;
          root = "/var/www/example.com";
        };
      };
      
      postgresql = {
        enable = true;
        package = pkgs.postgresql_15;
        ensureDatabases = [ "myapp" ];
        ensureUsers = [{
          name = "myapp";
          ensurePermissions = {
            "DATABASE myapp" = "ALL PRIVILEGES";
          };
        }];
      };
    };
    
    # Users
    users.users.alice = {
      isNormalUser = true;
      extraGroups = [ "wheel" "networkmanager" ];
      openssh.authorizedKeys.keys = [
        "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ... alice↯example.com"
      ];
    };
    
    # Environment
    environment = {
      systemPackages = with pkgs; [
        vim
        git
        curl
        wget
        htop
        tree
      ];
      
      variables = {
        EDITOR = "vim";
        BROWSER = "firefox";
      };
    };
  };
  
  # Home Manager configuration
  homeConfig = {
    home = {
      username = "alice";
      homeDirectory = "/home/alice";
      stateVersion = "23.05";
    };
    
    programs = {
      git = {
        enable = true;
        userName = "Alice";
        userEmail = "alice↯example.com";
        extraConfig = {
          init.defaultBranch = "main";
          pull.rebase = true;
        };
      };
      
      zsh = {
        enable = true;
        enableCompletion = true;
        enableAutosuggestions = true;
        enableSyntaxHighlighting = true;
        
        shellAliases = {
          ll = "ls -la";
          la = "ls -A";
          l = "ls -CF";
          grep = "grep --color=auto";
        };
        
        oh-my-zsh = {
          enable = true;
          theme = "robbyrussell";
          plugins = [ "git" "docker" "kubectl" ];
        };
      };
      
      tmux = {
        enable = true;
        terminal = "screen-256color";
        keyMode = "vi";
        extraConfig = ''
          set -g mouse on
          set -g status-bg colour235
          set -g status-fg colour136
        '';
      };
    };
    
    services = {
      gpg-agent = {
        enable = true;
        enableSshSupport = true;
        pinentryFlavor = "gtk2";
      };
    };
  };
  
  # Flake example
  flakeExample = {
    description = "A sample Nix flake";
    
    inputs = {
      nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
      flake-utils.url = "github:numtide/flake-utils";
    };
    
    outputs = { self, nixpkgs, flake-utils }:
      flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          packages.default = pkgs.stdenv.mkDerivation {
            pname = "my-app";
            version = "1.0.0";
            src = ./.;
            
            buildInputs = with pkgs; [ gcc ];
            
            buildPhase = ''
              gcc -o my-app main.c
            '';
            
            installPhase = ''
              mkdir -p $out/bin
              cp my-app $out/bin/
            '';
          };
          
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              gcc
              gdb
              valgrind
              pkg-config
            ];
            
            shellHook = ''
              echo "Welcome to the development environment!"
              echo "Available tools: gcc, gdb, valgrind"
            '';
          };
        });
  };
  
  # Advanced patterns
  advancedPatterns = {
    # Fixed-point combinators
    fix = f: let x = f x; in x;
    
    # Y combinator
    Y = f: (x: f (x x)) (x: f (x x));
    
    # Factorial using fix
    factorial = fix (f: n: if n <= 1 then 1 else n * f (n - 1));
    
    # Memoization
    memoize = f: let
      cache = {};
      memoized = x:
        if cache ? ${toString x}
        then cache.${toString x}
        else cache // { ${toString x} = f x; };
    in memoized;
    
    # Currying
    curry = f: x: y: f { inherit x y; };
    uncurry = f: { x, y }: f x y;
    
    # Composition
    compose = f: g: x: f (g x);
    pipe = x: fs: builtins.foldl' (acc: f: f acc) x fs;
    
    # Monadic operations
    maybe = {
      pure = x: { type = "just"; value = x; };
      nothing = { type = "nothing"; };
      bind = m: f: if m.type == "just" then f m.value else m;
      map = f: m: if m.type == "just" then { type = "just"; value = f m.value; } else m;
    };
  };
  
  # Comments and documentation
  # This is a single-line comment
  
  /*
    This is a multi-line comment
    that spans multiple lines
    and can contain /* nested */ comments
  */
  
  documentedFunction = 
    # This function adds two numbers
    # Arguments:
    #   x: first number
    #   y: second number
    # Returns: sum of x and y
    x: y: x + y;
  
  # Complex nested expressions
  complexExpression = let
    helper = x: y: z: 
      if x > 0 
      then helper (x - 1) (y + z) z
      else y;
  in helper 10 0 1;
  
  # Error handling patterns
  tryOr = default: expr: 
    if builtins.tryEval expr then expr else default;
  
  safeHead = list: 
    if builtins.length list > 0 
    then builtins.head list 
    else null;
  
  # Module system example
  moduleExample = { config, lib, pkgs, ... }: {
    options = {
      myService = {
        enable = lib.mkEnableOption "my custom service";
        port = lib.mkOption {
          type = lib.types.int;
          default = 8080;
          description = "Port to listen on";
        };
        package = lib.mkOption {
          type = lib.types.package;
          default = pkgs.myService;
          description = "Package to use for the service";
        };
      };
    };
    
    config = lib.mkIf config.myService.enable {
      systemd.services.myService = {
        description = "My Custom Service";
        wantedBy = [ "multi-user.target" ];
        serviceConfig = {
          ExecStart = "${config.myService.package}/bin/myservice --port ${toString config.myService.port}";
          Restart = "always";
          User = "myservice";
          Group = "myservice";
        };
      };
      
      users.users.myservice = {
        isSystemUser = true;
        group = "myservice";
      };
      
      users.groups.myservice = {};
      
      networking.firewall.allowedTCPPorts = [ config.myService.port ];
    };
  };

# Main expression - everything above is bound in this let expression
in {
  # Export all the defined values
  inherit intValue floatValue stringValue boolTrue boolFalse nullValue;
  inherit greeting multiLineString;
  inherit absolutePath relativePath homePath;
  inherit httpUrl gitUrl;
  inherit emptyList numberList mixedList nestedList;
  inherit emptySet simpleSet nestedSet recursiveSet dynamicKeys;
  inherit simpleFunction multiArgFunction functionWithDefault functionWithEllipsis;
  inherit destructuringFunction listFunction;
  inherit conditionalValue nestedConditional;
  inherit letExpression nestedLet withExpression inheritExample assertExample;
  inherit arithmetic comparison logical;
  inherit listOps setOps stringOps typeChecks;
  inherit builtinFunctions fileOps;
  inherit simpleDerivation packageWithDeps overlay;
  inherit nixosConfig homeConfig flakeExample;
  inherit advancedPatterns documentedFunction complexExpression;
  inherit tryOr safeHead moduleExample;
  
  # Additional computed values
  computedValues = {
    timestamp = builtins.currentTime;
    nixVersion = builtins.nixVersion;
    currentSystem = builtins.currentSystem;
    storeDir = builtins.storeDir;
  };
  
  # Test results
  testResults = {
    arithmeticTest = arithmetic.addition == 3;
    stringTest = stringOps.length == 5;
    listTest = builtins.length numberList == 5;
    functionTest = simpleFunction 5 == 6;
    conditionalTest = conditionalValue == "yes";
  };
  
  # Summary
  summary = "Nix expression language test completed successfully";
}