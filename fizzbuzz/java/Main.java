import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        //this syntax sucks, but I have yet to find an good way to do this using classes like in the c example, or tuples like the rust example
        Object[] sneed = {
            "Fizz", 3,
            "Buzz", 5,
        };

        for (int i = 1; i <= 25; i += 1) {
            boolean sneeded = false;
            for (int j = 0; j < sneed.length; j += 2) {
                if (i % (int)sneed[j+1] == 0) {//I dont like the casting here
                    System.out.print(sneed[j]);
                    sneeded = true;
                }
            }

            System.out.println(sneeded ? "" : i);
        }
        System.out.println(sneed[0]);
    }
}