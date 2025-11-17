if __name__ == "__main__":
    token = get_acess_token()

    if token:
        appy_classification(token, ENITITY_GUID, CLASSIF_NAME)
        else:
            print("Nãp foi possivel autenticas.")