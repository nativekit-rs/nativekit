View! {
    CustomGreeting(name: string = "Friend")

    VStack.spacing(4) {
        Text("Reusable Component!").font(.title1)
        Text("Hello, {}", name)
    }
}
