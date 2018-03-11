from Q import Q

prize = Q('Ethereum.csv').loadLines().cut(',',4,5).read()
cap = Q('Ethereum.csv').loadLines().cut(',',3,4).read()
supply = Q('Ethereum.csv').loadLines().cut(',',2,3).read()


fd = open('eth.csv','w')
for i in range(len(prize)):
    fd.write('%s,%s,%s\n' % (prize[i],cap[i],supply[i]))
fd.close()



