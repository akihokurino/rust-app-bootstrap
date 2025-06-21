## Bastion

### SSMエージェントの起動を確認

```shell
PUBLIC_IP=$(aws ec2 describe-instances --instance-ids i-0a1d4cc8452aa4d17 --query 'Reservations[0].Instances[0].PublicIpAddress' --output text)
aws ec2-instance-connect ssh --instance-id i-0a1d4cc8452aa4d17 --os-user ec2-user
sudo systemctl status amazon-ssm-agent
```