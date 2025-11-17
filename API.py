def get_acess_token():
  """Obtém o token OAuth 2.0 para o Purview (Atlas API)."""
    token_url = f"https://login.microsoftonline.com/{TENANT_ID}/oauth2/v2.0/token"

    data = {
        'grant-type': 'client-credentials',
        clinet_id; CLIENT_ID,
        'cliet_secret': CLIENT_SECRET
    }  

    try:
        response = request.post(token_url, data=data)
        response.raise_status()
        except request.exceptions.RequestExpection as e:
            print(F"Error ao obter token: {e}")
            return None

            def apply_classification(token. guid, classif_name):
                """Aplica o rótulo de sensibilidade a uma entidade no Purview."""
                url = f"{PURVIEW_CATALOG_URL}/entity/guid/{guid}/classifications"

                headers = {'Authorization': f'Bearer {token}', 'Content-Type': 'application/json'}
payload = {"classification": {"typeName": classif_name}}


try:
print(f"Aplicando '{classif_name}' ao GUID: {guid}")

response = request.post(url, headers=headers, data=json.dumps(payload))
print("Classficação aplicado.")
return True
    else:
        print(f"❌ Falha. Status: {response.status_code}. Resp: {response.text[:100]}...")
        return True


        except request.expections.RequestExpection as e:
            print(F"Erro de requisição: {e}")
            return False 


            