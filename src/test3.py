#import math
print("Welcome to Aiden and Tad's Fabulous Kinematics Calculator!")
print("Input the number of the equation you want")
print(" 1: v=vo+at")
print(" 2: v^2=vo^2+2a(xo+x)")
print(" 3: w=wo+αt")
print(" 4: w^2=wo^2+2a(Ɵ-Ɵi)")
print(" 5: V=IR")
print(" 6: ac=v^2/r")
print(" 7: Ek=1/2mv^2")
print(" 8: Us=1/2kx^2")
print(" 9: Ek=1/2iw^2")
print("10: Ug=mgh")
equation = input ("")
if equation == "1":
  print("Input the variables you know, and let me calculate the one you don't! Use x for unknown variable")
  print("v=vo+at")
  vel = input("v:")
  vi = input("vo:")
  acc = input("a:")
  time = input("t:")
  if vel == "x":
   print("Final Velocity =", float (vi)+float (acc)* float (time))
  if vi == "x":
   vt = float (acc)* float (time)
   print("Initial Velocity =", float (vel)-float (vt))
  if acc == "x":
   vv = float (vel)- float (vi)
   print("Acceleration =", float (vv)/float (time))
  if time == "x":
   print("Time =", (float (vel)- float (vi))/float (acc))

if equation == "2":
  print("Input the variables you know, and let me calculate the one you don't! Use x for unknown variable")
  print("v^2=vo^2+2a(xo+x)")
  vel = input("v:")
  vi = input("vo:")
  acc = input("a:")
  xi = input ("xo:")
  x = input("x:")
  if vel == "x":
    print("Final Velocity =", ((((float (xi)+float (x))*float (acc))*2)+(float (vi)**2))**.5)
  if vi == "x":
    print("Initial Velocity=", ((float (vel)**2)-(((float (x)-float (xi))*float (acc))*2))**.5)
  if acc == "x":
    print("Acceleration =", (float (vel)**2)/(((float (xi)+float (x))*2))+(float (vi)**2))
  if xi == "x":
    print("Initial Position =", ((((float (vel)**2)-(float (vi)**2))/(float (acc)*2))-float (x))*(1-2))
  if x == "x":
    print("Final Position =", (((float (vel)**2)-(float (vi)**2))/(float (acc)*2))+ float (xi))
if equation == "5":
  print("Input the variables you know, and let me calculate the one you don't! Use x for unknown variable")
  print("V=IR")
  volt = input("V:")
  cur = input("I:")
  res = input("R:")
  if res == "x":
   print("Resistance =", float (volt)/float (cur))
  if cur == "x":
   print ("Current =", float (volt)/float (res))
  if volt == "x":
   print( "Voltage =", float (cur)*float (res))
if equation == "3":
  print("Input the variables you know, and let me calculate the one you don't! Use x for unknown variable")
  print("w=wo+αt")
  rotvel = input("w:")
  rotvi = input("wo:")
  rotacc = input("α:")
  time = input("t:")
  if rotvel == "x":
   print("Final Velocity =", float (rotvi)+float (rotacc)* float (time))
  if rotvi == "x":
   print ("Initial Velocity =", float (rotvel)-(float (rotacc)* float (time)))
  if rotacc == "x":
   print ("Acceleration =", (float (rotvel)- float (rotvi))/float (time))
  if time == "x":
   print ("Time =", (float (rotvel)- float (rotvi))/float (rotacc))
if equation == "6":
  print ("Input the variables you know, and let me calculate the one you don't! Use x for unknown variable")
  print ("ac=v^2/r")
  acc = input("a:")
  vel = input("v:")
  rad = input("r:")
  if acc == "x":
    print ("Acceleration =", (float (vel)**2)/float (rad))
  if vel == "x":
    print ("Velocity =", float (acc)*float (rad))
  if rad == "x":
    print ("Radius =", (float (vel)**2)/float (acc))
if equation == "4":
  print ("Input the variables you know, and let me calculate the one you don't! Use x for unknown variable")
  print ("w^2=wo^2+2a(Ɵ-Ɵi)")
  vel = input("w:")
  vi = input("wo:")
  acc = input("α:")
  xi = input ("Ɵi:")
  x = input("Ɵ:")
  if vel == "x":
   print("Final Rotational Velocity =", (((float (xi)+float (x))*float (acc)*2)+(float (vi)**2))**.5)
  if vi == "x":
   print ("Initial Rotational Velocity =", ((float (vel)**2)-((float (xi)+float (x))*float (acc))*2)**.5)
  if acc == "x":
   print ("Rotational Acceleration =", (float (vel)**2)/(((float (xi))+(float (x)))*2)+(float (vi)**2))
  if xi == "x":
   print ("Initial Position =", ((((float (vel)**2)-(float (vi)**2))/(float (acc)*2))-float (x))*(1-2))
  if x == "x":
   print ("Final Position =", ((float (vel)**2)-(float (vi)**2))/(float (acc)*2)+float (xi))
if equation == "7":
  print ("Input the variables you know, and let me calculate the one you don't! Use x for unknown variable")
  print ("Ek=mv^2")
  vel = input("v:")
  mas = input("m:")
  ek = input("Ek:")
  if vel == "x":
    print ("Velocity =", ((float (ek)*2)/(float (mas)))**.5)
  if mas == "x":
    print ("Mass =", (float (ek)*2)/(float (vel)**2))
  if ek == "x":
    print ("Kinetic Energy =", ((float (mas)*.5)*(float (vel)**2)))
if equation == "10":
  print ("Input the variables you know, and let me calculate the one you don't! Use x for unknown variable")
  print ("Ug=mgh")
  Ug = input('Ug:')
  mass = input('m:')
  grav = input('g:')
  height = input('h:')
  if Ug == 'x':
     print('Gravitational Potential Energy =',float(mass)*float(grav)*float(height))
  if mass == 'x':
    gh = float(grav)*float(height)
    print('Mass =',float(Ug)/gh)
  if grav == 'x':
    mh = float(mass)*float(height)
    print('Gravitational Acceleration =',float(Ug)/float(mh))
  if height == 'x':
    mg = float(mass)*float(grav)
    print('Height =',float(Ug)/mg)
if equation == "8":
  print("Us=1/2kx^2")
  vel = input("x:")
  mas = input("k:")
  ek = input("Us:")
  if vel == "x":
    tek = int (ek)*2
    tekm = int (tek)/int (mas)
    print("Position =", int (tekm)**.5)
  if mas == "x":
    tek = int (ek)*2
    vels = int (vel)**2
    print("Spring Constant =", int (tek)/ int (vels))
  if ek == "x":
    hm = int (mas)*.5
    vels = int (vel)**2
    print("Elastic Potential Energy =", int (hm)*int (vels))
if equation == "9":
  print("Imput the variables you know, and let me calculate the one you don't! Use x for unknown variable")
  print("Ek=1/2iw^2")
  vel = input("w:")
  mas = input("i:")
  ek = input("Ek:")
  if vel == "x":
    tek = int (ek)*2
    tekm = int (tek)/int (mas)
    print("Angular Velocity =", int (tekm)**.5)
  if mas == "x":
    tek = int (ek)*2
    vels = int (vel)**2
    print("Rotational Inertia =", int (tek)/ int (vels))
  if ek == "x":
    hm = int (mas)*.5
    vels = int (vel)**2
    print("Rotational Kinetic Energy =", int (hm)*int (vels))
elif equation > "10":
  print("Sorry, invalid equation")
