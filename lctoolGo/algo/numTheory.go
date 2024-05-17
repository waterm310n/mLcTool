package algo

// 最大公约数,基于
// 
// 1.辗转相除法  Gcd(a,b) = Gcd(b%a,a)
// 
// 2.定义 Gcd(a,0) = a
// 
// 推论：Gcd(a1,a2,a3) = Gcd(a1,Gcd(a2,a3)) 
func Gcd(a, b int) int {
	for a != 0 {
		a, b = b%a, a
	}
	return b
}

// 最小公倍数，基于
// 
// 1. Lcm(a,b)*Gcd(a,b) = a*b
// 
// 推论：Lcm(a1,a2,a3) = Lcm(a1,Lcm(a2,a3)) 
func Lcm(a, b int) int {
	return (a * b) / Gcd(a, b)
}
