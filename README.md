https://stackoverflow.com/questions/68275460/default-credentials-can-not-be-used-to-assume-new-style-deployment-roles

# Deployment
```
cdk synth && cdk bootstrap --trust=ACCOUNT_ID --cloudformation-execution-policies=arn:aws:iam::aws:policy/AdministratorAccess --verbose && cdk deploy 
```