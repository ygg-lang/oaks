#!/bin/bash
# Comprehensive Bash Lexer Test

# Variables
NAME="World"
COUNT=10
PI=3.14

# Parameter Expansion
echo "Hello, ${NAME}!"
echo "Default: ${UNDEFINED:-DefaultValue}"
echo "Length: ${#NAME}"
echo "Substring: ${NAME:0:3}"
echo "Replacement: ${NAME/World/Bash}"

# Command Substitution
DATE=$(date +%Y-%m-%d)
FILES=`ls -la`

# Arithmetic Expansion
RESULT=$((COUNT + 5 * 2))
((COUNT++))

# Arrays
FRUITS=("Apple" "Banana" "Cherry")
echo "First fruit: ${FRUITS[0]}"
echo "All fruits: ${FRUITS[@]}"
declare -A DICT
DICT[key]="value"

# Conditionals
if [ "$COUNT" -gt 10 ]; then
    echo "Count is greater than 10"
elif [[ "$NAME" == "World" && -f "file.txt" ]]; then
    echo "Hello World and file exists"
else
    echo "Condition not met"
fi

# Case Statement
case "$1" in
    start)
        echo "Starting..."
        ;;
    stop)
        echo "Stopping..."
        ;;
    *)
        echo "Usage: $0 {start|stop}"
        exit 1
        ;;
esac

# Loops
for fruit in "${FRUITS[@]}"; do
    echo "I like $fruit"
done

for ((i=0; i<5; i++)); do
    echo "Number $i"
done

while [ $COUNT -gt 0 ]; do
    echo "Countdown: $COUNT"
    ((COUNT--))
done

until [ $COUNT -eq 5 ]; do
    ((COUNT++))
done

# Functions
function my_func() {
    local var="local"
    echo "Arguments: $1, $2"
    return 0
}

simple_func() {
    echo "Simple function"
}

# Redirection and Pipes
echo "Log" > log.txt
cat < input.txt >> output.txt
ls | grep "txt" | sort

# Heredocs
cat <<EOF
This is a heredoc.
Variables like $NAME are expanded.
EOF

cat <<'EOF'
This is a literal heredoc.
Variables like $NAME are NOT expanded.
EOF

# Process Substitution
diff <(ls dir1) <(ls dir2)

# Special Variables
echo "Script name: $0"
echo "Arguments: $@"
echo "Arg count: $#"
echo "Exit status: $?"
echo "PID: $$"

# Coprocesses
coproc my_proc {
    read line
    echo "Processed: $line"
}

# Brace Expansion
echo {A,B,C}{1,2,3}
mkdir -p /tmp/project/{src,test,docs}

# Comments
# This is a comment
: '
This is a
multiline comment
hack
'
