import sys


UPDATE_FMT = "{url}/api/v1/{room}/update_field/{i}"
GET_FMT = "{url}/api/v1/{room}/get_field/{i}"
RESULT_FMT = '<a href="{update}"><img src="{get}" width="100"/></a>'


def get_or_default(l: list, idx: int, default=None):
    return l[idx] if -len(l) <= idx < len(l) else default


def to_queries(queries):
    queries = filter(lambda i: i[1] is not None, queries.items())
    
    if not queries:
        return ""

    formatted = "&".join([f"{k}={v}" for k, v in queries])
    return "?" + formatted


def format_cell(url, room, i, redirect=None):
    get_url = GET_FMT.format(url=url, room=room, i=i)
    update_url = UPDATE_FMT.format(url=url, room=room, i=i)
    update_url = update_url + to_queries({"r": redirect})
    return RESULT_FMT.format(get=get_url, update=update_url)


def main(url, room, redirect):
    res = "|Tic|Tac|Toe|\n|-|-|-|\n"

    def fmt_cell(i, j):
        return format_cell(url, room, i * 3 + j, redirect)

    for i in range(3):
        cells = [fmt_cell(i, j) for j in range(3)]
        res += "|" + "|".join(cells) + "|\n"

    print(res)


if __name__ == "__main__":
    if len(sys.argv) not in [3, 4]:
        print(
            "Usage:\n"
            "\tpython create_md_table.py <url> <room>\n"
            "\tpython create_md_table.py <url> <room> <redirect_url>"
        )
        sys.exit(1)

    a = [1, 2, 3]

    url = sys.argv[1]
    room = sys.argv[2]
    redirect = get_or_default(sys.argv, 3)

    main(url, room, redirect)
