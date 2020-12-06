import java.nio.file.*;
import java.nio.charset.*;
import java.util.*;

class Solution {
    public static void main(String[] args) throws Exception {
        var filepath = Paths.get("day-6/input.txt");
        var content = Files.readString(filepath, StandardCharsets.US_ASCII);
        var groupsAnswers = content.split("\n\n");

        var total = 0;
        for (var groupAnswers : groupsAnswers) {
            var membersAnswers = groupAnswers.split("\n");
            var allAnswes = stringToChars(membersAnswers[0]);

            for (var i = 1; i < membersAnswers.length; ++i) {
                var answers = stringToChars(membersAnswers[i]);
                allAnswes.retainAll(answers);
            }
            
            total += allAnswes.size();
        }
        System.out.println(total); 
    }

    private static List<Character> stringToChars(String string) 
    {
        List<Character> chars = new ArrayList<>(); 
        for (var ch : string.toCharArray()) chars.add(ch); 
        return chars; 
    } 

}