import sys


UPDATE_FMT = "{url}/api/v1/{room}/update_field/{i}"
GET_FMT = "{url}/api/v1/{room}/get_field/{i}"
RESULT_FMT = '<a href="{update}"><img src="{get}" width="100"/></a>'


def format_cell(url, room, i):
    get_url = GET_FMT.format(url=url, room=room, i=i)
    update_url = UPDATE_FMT.format(url=url, room=room, i=i)
    return RESULT_FMT.format(get=get_url, update=update_url)


def main(url, room):
    res = "|Tic|Tac|Toe|\n|-|-|-|\n"

    for i in range(3):
        cells = [format_cell(url, room, i * 3 + j) for j in range(3)]
        res += "|" + "|".join(cells) + "|\n"
        
    print(res)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
