

for l in open('data/Ethereum.csv').readlines():
  s = l.split(',')
  usd = ''.join(s[4:]).replace('"','').strip()
  print '%s,%s,%s,%s,%s' % (s[0],s[1],s[2],s[3],usd)
