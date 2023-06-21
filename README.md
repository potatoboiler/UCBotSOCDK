https://stackoverflow.com/questions/68275460/default-credentials-can-not-be-used-to-assume-new-style-deployment-roles

# Deployment
```
cdk synth && cdk bootstrap --trust=ACCOUNT_ID --cloudformation-execution-policies=arn:aws:iam::aws:policy/<role_here> --verbose && cdk deploy 
```

Uses Zapier integration (gmail -> python trigger -> |AWS| -> APIG -> Lambda).