const rate = 1.1

let token_one = 1
let token_two = 1

for (let i = 1; i <= 300; i++) {
    if (i % 2 == 0) {
        console.log('time: ' + i)
        let gain  = (token_one * rate - token_one) / 2
        token_one = token_one - gain
        token_two = token_two + gain * 1.1
    } else {
        console.log('time: ' + i)
        let gain  = (token_two * rate - token_two) / 2
        token_two = token_two - gain
        token_one = token_one + gain * 1.1
    }
    console.log(token_one)
    console.log(token_two)
}