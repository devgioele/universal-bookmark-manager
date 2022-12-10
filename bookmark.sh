#!/bin/sh

#
# UNIVERSAL BOOKMARK MANAGER
#

##########
# CONFIG #
##########
# Menu cmd such that the first argument passed is taken as the prompt text.
alias menu="tofi --width 90% --height 90% --font-size 16 --prompt-text"
# Menu cmd such that the first argument passed is taken as the prompt text,
# and to be used as input field.
# Despite no match, the input is printed to stdout.
alias menu-input="tofi --require-match false --font-size 16 --padding-top 25 --padding-bottom 25 --padding-left 35 --padding-right 25 --prompt-text"
file="${XDG_DATA_HOME:-$HOME/.local/share}/bookmarking/bookmarks"
##########

# Create necessary dirs
mkdir -p "$(dirname $file)"

alias help="echo \"Choose 'add', 'remove' or 'pick'.\""
alias bookmark_to_pick="cat $file | menu 'Pick: '"
alias bookmark_to_remove="cat $file | menu 'Remove: '"

# Asks the user for tags recursively,
# listing what tags exist so far.
ask_tag() {
  new_tag="$(printf "$existing_tags" | menu-input "Tag $n: " | tr ' ' '-')"
  if [ -n "$new_tag" ]
  then
    new_tags="$(printf "$tags\n#$new_tag")"
    # If the added tag is new
    if [ -z "$(printf "$new_tags" | sort | uniq -D)" ]
    then
      existing_tags="$(printf "$existing_tags$new_tag" | sort -u)\n"
      tags="$(printf "$new_tags" | sort -u)"
      n="$((n+1))"
    fi
    ask_tag
  fi
}

if [ "$#" -ne 1 ]
then
  echo "Invalid number of arguments."
  help
  exit 1
fi

if [ "$1" = "add" ]
then
  # Add the current selection to the file
  bookmark="$(wl-paste --primary 2> /dev/null)"
  if [ -n "$bookmark" ]
  then
    # Verify correctness of bookmark
    if printf "$bookmark" | grep " "
    then
      notify-send "Cannot copy invalid bookmark!" "Spaces are not allowed."
      exit 1
    fi
    # Verify existence of bookmark, ignoring any tags
    if grep -q "^$bookmark" "$file"
    then
      notify-send "Already bookmarked!"
    else
      # Prompt for tags, listing existing tags
      existing_tags="$(cat "$file" | grep -Po "(?<= #)[^ ]+" | sort -u)"
      # `sort` does not append a newline to the last line when printing to stdout.
      # Append the newline if the string is not empty.
      if [ -n "$existing_tags" ]
      then
        existing_tags="$existing_tags\n"
      fi
      tags=''
      n=1
      ask_tag
      bookmark_with_tags="$bookmark$(printf "$tags" | tr '\n' ' ')"
      echo "$bookmark_with_tags" >> "$file"
      notify-send "Bookmark added!" "$bookmark"
    fi
  else
    notify-send "Clipboard is empty!" "Nothing to bookmark"
  fi
elif [ "$1" = "remove" ]
then
  # Remove the chosen bookmark from the file
  bookmark_with_tags="$(bookmark_to_remove)"
  if [ -n "$bookmark_with_tags" ]
  then
    if sed -i "\|^$bookmark_with_tags$|d" "$file"
    then
      notify-send "Bookmark removed!" "$bookmark_with_tags"
    else
      notify-send "Bookmark does not exist!" "$bookmark_with_tags"
    fi
  fi
elif [ "$1" = "pick" ]
then
  # Let the user choose a bookmark and copy the chosen bookmark
  bookmark_with_tags="$(bookmark_to_pick)"
  if [ -n "$bookmark_with_tags" ]
  then
    if grep -q "^$bookmark_with_tags$" "$file"
    then
      # Copy the bookmark without the tags
      printf "$bookmark_with_tags" | grep -Eo "^[^ ]+" | tr -d '\n' | wl-copy
      notify-send "Bookmark copied!"
    else
      notify-send "Bookmark does not exist!" "$bookmark_with_tags"
    fi
  fi
else
  echo "Unknown option!"
  help
fi
