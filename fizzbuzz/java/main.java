import java.util.ArrayList;

public class main {
    public static void main(String[] args) {
        Object[] sneed = {
            "Fizz", 3,
            "Buzz", 5,
        };

        for (int i = 1; i <= 25; i += 1) {
            boolean sneeded = false;
            for (int j = 0; j < sneed.length; j += 2) {
                if (i % (int)sneed[j+1] == 0) {
                    System.out.print(sneed[j]);
                    sneeded = true;
                }
            }

            System.out.println(sneeded ? "" : i);
        }
        System.out.println(sneed[0]);
    }
}