import requests
payload=dict(
    type="in",
    qrCode="adfads",
    weight=97987.9,
    unit="吨",
    number=1,
)
host=""
url=f"http://{host}/rest/iot/in-out"
req=requests.post(url,json=payload)
print (req.text)