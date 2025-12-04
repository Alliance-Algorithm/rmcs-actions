import psutil


def main():
    net_if_addrs = psutil.net_if_addrs()


if __name__ == "__main__":
    main()
