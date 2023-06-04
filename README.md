# Discord facts bot
Made for a code jam. Currently in progress. It will be hosted and made available for adding to servers when complete.

### Bot usage
Add the bot to your sever (Invite link is tba) and in a channel it can see, use `/help` (make sure the facts bot is selected) to see a list of availible commands.
There is plenty of different facts to be learned! 

### Host the bot from source code
1. Clone this repository to a folder
2. Make sure you have installed cargo as per the rust website's directions
3. Add the required token files and fill them with the relevant api token. Ensure no spaces or newlines - it must be exactly the api key. At time of writing they are 
   - `discord.file` A discord bot token created in the developer portal
   - `api-ninjas-com-key.file` A api-ninjas.com token which is provided to you when you make an account there.
**You may need others.** you can look in `.gitignore` to make an educated guess on which files to create. **If you do not provide the correct files, the bot may crash** when certain commands are run. This will hopefully be improved in future!
4. Use `cargo build` to download all the dependencies and make sure everything is in order before starting the bot. This may take a long time! Make sure you have coffee at the ready. (This step is optional: if you skip this step, all of this will be done in the next one). 
5. Run the bot with `cargo run`
