import argparse
import ssl
import urllib.request
import sys
import os


def show_progress(xfered, size):
    percent = float(xfered) / size * 100
    print("Downloading: %.0f%%" % percent, flush=True, end="\r")


def download(url, output_file):
    context = ssl.create_default_context()
    url_obj = urllib.request.urlopen(url, context=context)

    content_length = url_obj.headers.get("content-length")
    size = int(content_length)
    buff_size = 100 * 1024
    xferd = 0

    dest_file = open(output_file, "wb")
    try:
        while True < size:
            data = url_obj.read(buff_size)
            if not data:
                break
            xferd += len(data)
            show_progress(xferd, size)
            dest_file.write(data)
        if xferd != size:
            # short read :/
            sys.exit("Error: expecting {}, got {}".format(xferd, size))
    finally:
        dest_file.close()


def select_path_entry():
    entries = os.environ.get("PATH").split(os.path.pathsep)
    print("Heres are the possible locations to install dmenv")
    print("Select one element in the list")
    for i, entry in enumerate(entries, start=1):
        print("%2d" % i, entry)
    answer = input("> ")
    entry = None
    while True:
        try:
            num = int(answer)
            entry = entries[num - 1]
            return entry
        except ValueError:
            print("Please enter a number")
        except IndexError:
            print("Please choose between 0 and", len(entries))
        answer = input("> ")


def on_existing_dest(dest, *, upgrade=False):
    if upgrade:
        os.remove(dest)
    else:
        sys.exit("Error: %s already exists. Use --upgrade to upgrade" % dest)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dest")
    parser.add_argument("--upgrade", action="store_true")
    args = parser.parse_args()

    url = "https://dmerej.info/pub/dmenv-%s" % sys.platform
    if sys.platform == "windows":
        url += ".exe"
        out = "dmenv.exe"
    else:
        out = "dmenv"

    if args.dest:
        dest = args.dest
    else:
        path_entry = select_path_entry()
        dest = os.path.join(path_entry, out)

    if os.path.exists(dest):
        on_existing_dest(dest, upgrade=args.upgrade)

    print("Downloading", url, "to", dest)
    download(url, dest)
    os.chmod(dest, 0o755)


if __name__ == "__main__":
    main()
