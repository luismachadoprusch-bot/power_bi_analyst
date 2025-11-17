import requests
import json
import os # Uso de variáveis de ambiente para credenciais

# --- Configurações (Ajuste ou use variáveis de ambiente) ---
TENANT_ID = os.environ.get("AZURE_TENANT_ID", "SEU_TENANT_ID")
CLIENT_ID = os.environ.get("AZURE_CLIENT_ID", "SEU_CLIENT_ID")
CLIENT_SECRET = os.environ.get("AZURE_CLIENT_SECRET", "SEU_CLIENT_SECRET")
PURVIEW_ACCOUNT = "sua-conta-purview"
# GUID (ID do ativo no Catálogo Purview) e o Rótulo (Classification)
ENTITY_GUID = "c0888888-0000-0000-0000-000000000000" 
CLASSIF_NAME = "LGPD_DadosPessoaisSensíveis"

# URL Base: Mantida curta
PURVIEW_CATALOG_URL = f"https://{PURVIEW_ACCOUNT}.purview.azure.com/catalog/api/atlas/v2"