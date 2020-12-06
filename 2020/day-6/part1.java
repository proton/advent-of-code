import java.nio.file.*;
import java.nio.charset.*;
import java.util.*;

class Part1 {
    public static void main(String[] args) throws Exception {
        var filepath = Paths.get("day-6/input.txt");
        var content = Files.readString(filepath, StandardCharsets.US_ASCII);
        var groupAnswers = content.split("\n\n");

        var total = 0;
        for (var i = 0; i < groupAnswers.length; ++i) {
            var answers = groupAnswers[i].replace("\n", "");
            var uniqAnswers = uniqueCharacters(answers);
            total += uniqAnswers.size();
        } 
        System.out.println(total); 
    }

    private static Set<Character> uniqueCharacters(String string) {
        var set = new HashSet<Character>();
        for (var i = 0; i < string.length(); ++i)
            set.add(string.charAt(i));
        return set;
    }
}