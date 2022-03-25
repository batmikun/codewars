import re


def main():
    result = to_camel_case("the_stealth_warrior")

    print(result)


def to_camel_case(text):

    if(text == ""):
        return ""

    list_of_words = re.split("\W|_|-", text)

    result = list_of_words[0]

    for word in list_of_words[1:]:

        result += word.capitalize()

    return result


main()
